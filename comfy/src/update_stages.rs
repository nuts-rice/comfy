use crate::*;

pub fn run_early_update_stages(c: &mut EngineContext) {
    let delta = delta();

    {
        let mut state = GLOBAL_STATE.borrow_mut();

        state.fps = (1.0 / delta).round() as i32;
        state.egui_scale_factor = egui().pixels_per_point();
    }

    dev_hotkeys(c);

    // Clear all the lights from previous frame
    LightingState::begin_frame();

    process_asset_queues(c);

    if !*c.is_paused.borrow() {
        set_unpaused_time(get_unpaused_time() + delta as f64);
    }

    render_text(c);
    update_blood_canvas(c);
    update_camera(c);

    // TODO: not ideal
    clear_background(BLACK);

    if is_key_pressed(KeyCode::Backquote) &&
        is_key_down(KeyCode::LCtrl) &&
        is_key_down(KeyCode::LAlt)
    {
        let mut config = game_config_mut();
        config.dev.show_debug = !config.dev.show_debug;
    }

    lighting_parameters_window(c);
    update_child_transforms();

    run_mid_update_stages(c);
}

fn run_mid_update_stages(c: &mut EngineContext) {
    timings_add_value("delta", delta());

    pause_system(c);
    point_lights_system();

    if is_key_pressed(KeyCode::F6) {
        GlobalParams::toggle_flag("debug");
    }
}

// TODO: Some of the ordering in the update stages is definitely incorrect.
pub fn run_late_update_stages(c: &mut EngineContext, delta: f32) {
    update_animated_sprites(c);
    update_trails(c);
    update_drawables(c);
    process_sprite_queue();
    process_temp_draws(c);
    combat_text_system();
    process_notifications(c);
    show_errors(c);
    update_perf_counters(c);
    show_lighting_ui(c);

    c.draw.borrow_mut().marks.retain_mut(|mark| {
        mark.lifetime -= delta;
        mark.lifetime > 0.0
    });

    for mark in c.draw.borrow().marks.iter() {
        draw_circle_z(
            mark.pos.to_world(),
            0.1,
            mark.color,
            90,
            TextureParams {
                blend_mode: BlendMode::Alpha,
                ..Default::default()
            },
        );
    }

    player_follow_system();
    animated_sprite_builder_check();
    renderer_update(c);

    let is_paused =
        *c.is_paused.borrow() || c.flags.borrow_mut().contains(PAUSE_DESPAWN);

    if !is_paused {
        for (entity, despawn) in world_mut().query_mut::<&mut DespawnAfter>() {
            despawn.0 -= delta;

            if despawn.0 <= 0.0 {
                c.to_despawn.borrow_mut().push(entity);
            }
        }
    }

    main_camera_mut().update(delta);
    commands().run_on(&mut world_mut());
    world_mut().flush();
}

fn dev_hotkeys(_c: &EngineContext) {
    // TODO: get rid of this & move it to nanovoid instead
    if is_key_pressed(KeyCode::F1) {
        let mut global_state = GLOBAL_STATE.borrow_mut();
        global_state.mouse_locked = !global_state.mouse_locked;
    }

    // TODO: make this configurable
    // renderer
    //     .window()
    //     .set_cursor_visible(global_state.mouse_locked);

    if is_key_pressed(KeyCode::F7) {
        let mut config = game_config_mut();

        config.dev.show_lighting_config = !config.dev.show_lighting_config;
        config.dev.show_buffers = !config.dev.show_buffers;
    }

    if is_key_pressed(KeyCode::F8) {
        let mut config = game_config_mut();

        config.dev.show_fps = !config.dev.show_fps;
    }

    #[cfg(feature = "exit-after-startup")]
    if get_time() > 1.2 {
        std::process::exit(0);
    }

    let _span = span!("game-state update");

    #[cfg(any(feature = "quick-exit", feature = "dev"))]
    if is_key_down(KeyCode::F1) && is_key_pressed(KeyCode::Escape) {
        println!("fast exit");
        std::process::exit(0);
    }
}

fn process_asset_queues(c: &mut EngineContext) {
    ASSETS.borrow_mut().process_asset_queues();

    AudioSystem::process_sounds();

    // TODO: this is ugly but would otherwise need an extra channel since
    //       AssetLoader doesn't have access to WgpuRenderer
    if let Some(mut guard) =
        ASSETS.borrow_mut().asset_loader.wgpu_load_queue.try_lock()
    {
        if let Some(batch) = guard.take() {
            for item in batch.into_iter() {
                c.renderer.loaded_image_send.send(item).log_err();
            }
        }
    }

    // if let Some(texture_queue) =
    //     ASSETS.borrow_mut().asset_loader.wgpu_load_queue.lock().take()
    // {
    //     for item in texture_queue.into_iter() {
    //         c.renderer.loaded_image_send.send(item).log_err();
    //     }
    // }
}

fn render_text(_c: &mut EngineContext) {
    let _span = span!("text");

    let painter = egui().layer_painter(egui::LayerId::new(
        egui::Order::Background,
        egui::Id::new("text-painter"),
    ));

    let text_queue =
        GLOBAL_STATE.borrow_mut().text_queue.drain(..).collect_vec();

    for text in text_queue {
        let align = match text.align {
            TextAlign::TopLeft => egui::Align2::LEFT_TOP,
            TextAlign::Center => egui::Align2::CENTER_CENTER,
            TextAlign::TopRight => egui::Align2::RIGHT_TOP,
            TextAlign::BottomLeft => egui::Align2::LEFT_BOTTOM,
            TextAlign::BottomRight => egui::Align2::RIGHT_BOTTOM,
        };

        // TODO: maybe better way of doing this?
        let screen_pos =
            text.position.as_world().to_screen() / egui_scale_factor();

        painter.text(
            egui::pos2(screen_pos.x, screen_pos.y),
            align,
            text.text,
            text.font,
            text.color.egui(),
        );
    }
}

fn update_blood_canvas(_c: &mut EngineContext) {
    let _span = span!("blood_canvas");

    // TODO: this really doesn't belong here
    blood_canvas_update_and_draw(|key, block| {
        draw_sprite_ex(
            block.handle,
            (key.as_vec2() + splat(0.5)) * blood_block_world_size() as f32,
            WHITE,
            Z_BLOOD_CANVAS,
            DrawTextureParams {
                dest_size: Some(
                    splat(blood_block_world_size() as f32).as_world_size(),
                ),
                blend_mode: BlendMode::Alpha,
                ..Default::default()
            },
        );
    });
}

fn update_camera(c: &mut EngineContext) {
    let _span = span!("update_camera");

    let mut global_state = GLOBAL_STATE.borrow_mut();
    let mut camera = main_camera_mut();

    let width = c.renderer.width();
    let height = c.renderer.height();

    camera.aspect_ratio = width / height;
    global_state.screen_size = vec2(width, height);

    let viewport = camera.world_viewport();

    let flipped_mouse_pos = vec2(
        global_state.mouse_position.x,
        global_state.screen_size.y - global_state.mouse_position.y,
    );

    let normalized = flipped_mouse_pos / global_state.screen_size * viewport -
        viewport / 2.0;

    if !global_state.mouse_locked {
        global_state.mouse_world = normalized + camera.center;
    }
}

fn update_child_transforms() {
    let mut transforms = HashMap::new();

    for (entity, transform) in world_mut().query_mut::<&Transform>() {
        transforms.insert(entity, *transform);
    }

    for (_, transform) in world().query::<&mut Transform>().iter() {
        let parent = if let Some(parent) = transform.parent {
            transforms
                .get(&parent)
                .cloned()
                .unwrap_or(Transform::position(Vec2::ZERO))
        } else {
            Transform::position(Vec2::ZERO)
        };

        let combined = transform.compose_with_parent(&parent);

        transform.abs_position = combined.position;
        transform.abs_rotation = combined.rotation;
        transform.abs_scale = combined.scale;
    }
}

fn update_trails(c: &mut EngineContext) {
    let _span = span!("trails");

    for (_, (trail, transform)) in
        world_mut().query_mut::<(&mut Trail, &Transform)>()
    {
        if !*c.is_paused.borrow() {
            trail.update(transform.position, c.delta);
        }
        trail.draw_mesh();
    }
}

fn point_lights_system() {
    for (_, (transform, light)) in
        world_mut().query_mut::<(&Transform, &PointLight)>()
    {
        draw_light(Light::simple(
            transform.position,
            light.radius * light.radius_mod,
            light.strength * light.strength_mod,
        ))
    }
}

fn update_animated_sprites(c: &mut EngineContext) {
    let mut call_queue = vec![];

    if !*c.is_paused.borrow() {
        for (entity, sprite) in world().query::<&mut AnimatedSprite>().iter() {
            if sprite.state.update_and_finished(c.delta) {
                commands().despawn(entity);

                // TODO: maybe not needed? replace with option
                let mut temp: ContextFn = Box::new(|_| {});
                std::mem::swap(&mut sprite.on_finished, &mut temp);

                call_queue.push(temp);
            }
        }
    }

    for call in call_queue.drain(..) {
        call(c);
    }
}

fn pause_system(c: &mut EngineContext) {
    // TODO: configurable pause
    if is_key_pressed(KeyCode::Escape) {
        if *c.is_paused.borrow() && *c.show_pause_menu {
            info!("Resuming");
            *c.is_paused.borrow_mut() = false;
            *c.show_pause_menu = false;
        } else if !*c.is_paused.borrow() && !*c.show_pause_menu {
            info!("Pausing");
            *c.is_paused.borrow_mut() = true;
            *c.show_pause_menu = true;
        } else {
            info!("Nothing");
        }
    }

    if !*c.is_paused.borrow() && !c.flags.borrow().contains(PAUSE_PHYSICS) {
        cooldowns().tick(c.delta);
        notifications().tick(c.delta);
    }
}

fn update_drawables(c: &mut EngineContext) {
    let _span = span!("drawables");

    let mut temp_drawables = vec![];

    std::mem::swap(&mut temp_drawables, &mut c.draw_mut().drawables);

    temp_drawables.retain_mut(|drawable| {
        if let Some(ref mut time) = drawable.time {
            if *time <= 0.0 {
                false
            } else {
                *time -= c.delta;
                (drawable.func)(c);
                true
            }
        } else {
            (drawable.func)(c);
            false
        }
    });

    let mut draw = c.draw_mut();
    std::mem::swap(&mut temp_drawables, &mut draw.drawables);

    for drawable in temp_drawables.drain(..) {
        draw.drawables.push(drawable);
    }
}

fn process_sprite_queue() {
    let _span = span!("sprite_queue");

    let world = world();
    let mut sprite_query = world.query::<(&Sprite, &Transform)>();
    let sprite_iter = sprite_query
        .iter()
        .map(|(_, (sprite, transform))| sprite.to_quad_draw(transform));

    let mut animated_sprite_query =
        world.query::<(&AnimatedSprite, &Transform)>();

    let animated_sprite_iter = animated_sprite_query
        .iter()
        .map(|(_, (sprite, transform))| sprite.to_quad_draw(transform));

    for (z_index, group) in sprite_iter
        .chain(animated_sprite_iter)
        .group_by(|draw| draw.z_index)
        .into_iter()
        .sorted_by_key(|(z_index, _)| *z_index)
    {
        // for draw in group.sorted_by(|a, b| a.texture.cmp(&b.texture)) {
        for draw in group {
            draw_sprite_ex(
                draw.texture,
                draw.transform.position,
                draw.color,
                z_index,
                DrawTextureParams {
                    source_rect: draw.source_rect,
                    dest_size: Some(draw.dest_size.as_world_size()),
                    rotation: draw.transform.rotation,
                    blend_mode: draw.blend_mode,
                    flip_x: draw.flip_x,
                    flip_y: draw.flip_y,
                    ..Default::default()
                },
            );
        }
    }
}

fn process_temp_draws(c: &mut EngineContext) {
    let _span = span!("temp draws");

    for texture in c.draw_mut().textures.drain(..) {
        draw_sprite_ex(
            texture.texture,
            texture.position.to_world(),
            texture.color,
            20, // TODO
            texture.params,
        );
    }

    for (position, radius, color) in c.draw_mut().circles.drain(..) {
        draw_circle(position.to_world(), radius, color, 200);
    }

    for line in c.draw_mut().lines.drain(..) {
        draw_line(
            line.start.to_world(),
            line.end.to_world(),
            line.width,
            line.color,
            70,
        );
    }

    // TODO: calculate world space font size
    for (text, position, color, size) in c.draw_mut().texts.drain(..) {
        draw_text_ex(&text, position, TextAlign::Center, TextParams {
            color,
            font: egui::FontId::new(size, egui::FontFamily::Monospace),
            ..Default::default()
        });
    }
}

fn process_notifications(_c: &mut EngineContext) {
    let notifications = notifications();

    if !notifications.notifications.is_empty() {
        egui::Window::new("Notifications")
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(20.0, 280.0))
            .resizable(false)
            .title_bar(false)
            .frame(egui::Frame::default())
            .collapsible(false)
            .show(egui(), |ui| {
                let _background_shape = ui.painter().add(egui::Shape::Noop);
                ui.add_space(18.0);

                ui.vertical_centered(|ui| {
                    for notification in notifications.notifications.iter() {
                        let font = egui::FontId::new(
                            16.0,
                            egui::FontFamily::Proportional,
                        );

                        ui.add(egui::Label::new(
                            egui::RichText::new(&notification.text)
                                .font(font)
                                .color(GOLD.egui()),
                        ));

                        // ui.colored_label(GREEN.egui(), &notification.text);
                    }
                });
                ui.add_space(28.0);

                // let bg = nine_patch_rect_ex(
                //     egui::Rect::from_min_size(
                //         ui.clip_rect().left_top(),
                //         ui.min_size(),
                //     ),
                //     c.cached_loader,
                //     egui(),
                //     "panel-horizontal",
                // );

                // ui.painter().set(background_shape, bg);
            });
    }
}

fn show_errors(_c: &mut EngineContext) {
    if cfg!(feature = "dev") &&
        game_config().dev.recording_mode == RecordingMode::None
    {
        let errors = ERRORS.borrow();

        if !errors.data.is_empty() {
            egui::Window::new("Errors")
                .anchor(egui::Align2::LEFT_TOP, egui::vec2(20.0, 20.0))
                .show(egui(), |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for (_, value) in ERRORS.borrow().data.iter() {
                            ui.colored_label(RED.egui(), value.as_ref());
                        }
                    });
                });
        }
    }
}

fn update_perf_counters(c: &mut EngineContext) {
    if cfg!(not(feature = "ci-release")) && game_config().dev.show_fps {
        let _span = span!("perf counters");

        let dt = c.dt_stats.next(frame_time());
        let fps = c.fps_stats.next(1.0 / frame_time());

        egui::Window::new("Performance")
            .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(0.0, 0.0))
            .default_width(250.0)
            .show(egui(), |ui| {
                let real_fps = get_fps();

                let fps_color = if real_fps < 55 { RED } else { WHITE };
                ui.colored_label(
                    fps_color.egui(),
                    format!("FPS (real): {:.0}", real_fps),
                );

                ui.separator();

                ui.label(format!(
                    "Delta mean: {:.0} ms ... std: {:.1} ms",
                    dt.mean * 1000.0,
                    dt.std_dev * 1000.0
                ));

                ui.label(format!(
                    "FPS mean: {:.0} ... std: {:.1}",
                    fps.mean, fps.std_dev
                ));

                ui.label("percentiles:");
                ui.label(format!("   50th: {:.0}", fps.percentile_50));
                ui.label(format!("   75th: {:.0}", fps.percentile_75));
                ui.label(format!("   90th: {:.0}", fps.percentile_90));
                ui.label(format!("   95th: {:.0}", fps.percentile_95));
                ui.label(format!("   99th: {:.0}", fps.percentile_99));

                ui.separator();
                if let Some(game_loop) = c.game_loop {
                    game_loop.lock().performance_metrics(&mut world_mut(), ui);
                }

                ui.separator();

                let mut particles = 0;

                for (_, particle_system) in
                    world_mut().query_mut::<&ParticleSystem>()
                {
                    particles += particle_system.particles.len();
                }

                // TODO: count collision events again?
                // let mut collision_events = 0;

                // for (_, collisions) in
                //     c.world().query::<&Collisions>().iter()
                // {
                //     collision_events += collisions.0.len();
                // }

                ui.label(format!("Lights: {}", light_count()));
                ui.label(format!("Particles: {}", particles));
                ui.label(format!("Entities: {}", world().len()));
                // ui.label(format!("Collision Events: {}", collision_events));

                ui.separator();

                ui.label("Perf Counters");

                ui.separator();

                for (counter_name, counter) in PerfCounters::global()
                    .counters
                    .iter()
                    .sorted_by(|a, b| a.0.cmp(b.0))
                {
                    ui.label(format!(
                        "{:<15}: {:<15.0}",
                        counter_name, counter.decayed_average,
                    ));
                }
                ui.separator();

                #[cfg(all(
                    feature = "memory-stats",
                    not(target_arch = "wasm32")
                ))]
                {
                    let _span = span!("memory_stats");

                    if let Some(usage) = memory_stats::memory_stats() {
                        ui.label(format!(
                            "Physical Mem: {} MB",
                            usage.physical_mem / (1024 * 1024)
                        ));
                        ui.label(format!(
                            "Virtual Mem: {} MB",
                            usage.virtual_mem / (1024 * 1024)
                        ));
                    } else {
                        ui.label(format!(
                            "Couldn't get the current memory usage :("
                        ));
                    }
                }

                #[cfg(feature = "jemalloc")]
                {
                    let _span = span!("jemalloc stats");
                    ui.separator();

                    ui.label("jemalloc");

                    jemalloc_ctl::epoch::advance().unwrap();

                    let allocated =
                        jemalloc_ctl::stats::allocated::read().unwrap();
                    let resident =
                        jemalloc_ctl::stats::resident::read().unwrap();
                    ui.label(format!(
                        "{} MB allocated",
                        allocated / (1024 * 1024)
                    ));
                    ui.label(format!(
                        "{} MB resident",
                        resident / (1024 * 1024)
                    ));
                }

                egui::Grid::new("timings-grid").show(ui, |ui| {
                    for (name, entry) in timings()
                        .data
                        .iter()
                        .sorted_by_key(|(_, entry)| entry.time)
                    {
                        let mean = if !entry.history.is_empty() {
                            entry.history.sum() / entry.history.len() as f32
                        } else {
                            0.0
                        };

                        ui.label(*name);
                        ui.label(format!("{:.03} ms", mean * 1000.0));

                        ui.end_row();
                    }
                });
            });
    }

    perf_counters_new_frame(c.delta as f64);
}

pub fn lighting_parameters_window(_c: &EngineContext) {
    if GlobalParams::flag("debug") {
        egui::Window::new("Parameters")
            .anchor(egui::Align2::LEFT_BOTTOM, egui::vec2(150.0, -80.0))
            .show(egui(), |ui| {
                let mut params = GLOBAL_PARAMS.borrow_mut();

                let floats = [
                    "filter-cutoff",
                    "filter-resonance",
                    "colorScale",
                    "exposure",
                    "---",
                    "bloomThreshold",
                    "bloom-lerp",
                    "bloomGamma",
                    "---",
                    "brightness",
                    "saturation",
                    "contrast",
                    "---",
                    "gamma",
                    "---",
                    "chromatic_aberration",
                ];

                for name in floats.iter() {
                    if *name == "---" {
                        ui.separator();
                    } else {
                        ui.horizontal(|ui| {
                            let range = params.floats.get_mut(name).unwrap();
                            ui.label(*name);

                            ui.add(
                                egui::DragValue::new(&mut range.value)
                                    .speed(range.speed)
                                    .clamp_range(range.min..=range.max),
                            );
                        });
                    }
                }

                ui.separator();

                let ints = ["bloom_alg", "physics_substeps", "tonemapping_alg"];

                for name in ints.iter() {
                    if *name == "---" {
                        ui.separator();
                    } else {
                        ui.horizontal(|ui| {
                            let range = params.ints.get_mut(name).unwrap();
                            ui.label(*name);

                            ui.add(
                                egui::DragValue::new(&mut range.value)
                                    .speed(0.1)
                                    .clamp_range(range.min..=(range.max - 1)),
                            );
                        });
                    }
                }
            });
    }
}

pub fn count_to_color(count: i32) -> Color {
    match count {
        0 => WHITE,
        1 => BLUE,
        2 => GREEN,
        3 => RED,
        4 => PURPLE,
        _ => YELLOW,
    }
}

fn player_follow_system() {
    for (_, (player_t, _)) in world().query::<(&Transform, &PlayerTag)>().iter()
    {
        // TODO; check that there is only one?

        for (_, (transform, _)) in
            world().query::<(&mut Transform, &FollowPlayer)>().iter()
        {
            transform.position = player_t.position;
        }
    }
}

fn animated_sprite_builder_check() {
    #[cfg(not(feature = "ci-release"))]
    for (entity, (_, transform)) in
        world_mut().query_mut::<(&AnimatedSpriteBuilder, Option<&Transform>)>()
    {
        error!(
            "AnimatedSpriteBuilder found in ECS (entity = {:?}), make sure to \
             call .build()",
            entity
        );

        if let Some(transform) = transform {
            draw_circle(transform.position, 0.5, RED, 499);
            draw_text(
                "AnimatedSpriteBuilder in ECS",
                transform.position,
                WHITE,
                TextAlign::Center,
            );
        }
    }
}

fn renderer_update(c: &mut EngineContext) {
    let delta = delta();

    SINGLE_PARTICLES.borrow_mut().retain_mut(|particle| {
        particle.update(delta);
        particle.lifetime_current > 0.0
    });

    // TODO: keep the same vec between frames
    let mut all_particles = Vec::new();

    for (_, (transform, particle_system)) in
        world_mut().query_mut::<(&Transform, &mut ParticleSystem)>()
    {
        particle_system.update(transform.position, delta);

        let err_texture = texture_id("error");

        if particle_system.start_when_texture_loaded {
            all_particles.extend(
                particle_system
                    .particles
                    .iter()
                    .filter(|p| {
                        p.lifetime_current > 0.0 && p.texture != err_texture
                    })
                    .cloned(),
            )
        } else {
            all_particles.extend(
                particle_system
                    .particles
                    .iter()
                    .filter(|p| p.lifetime_current > 0.0)
                    .cloned(),
            )
        }
    }

    let particle_queue = SINGLE_PARTICLES
        .borrow_mut()
        .iter()
        .chain(all_particles.iter())
        .map(|p| {
            ParticleDraw {
                position: (p.position + p.offset).extend(p.z_index as f32),
                rotation: p.rotation,
                texture: p.texture,
                // color: p.color,
                color: p.current_color(),
                size: p.size * p.current_size(),
                source_rect: p.source_rect,
                blend_mode: p.blend_mode,
            }
        })
        .collect_vec();

    let clear_color = GLOBAL_STATE.borrow_mut().clear_color;
    let frame_params =
        FrameParams { frame: get_frame(), delta, time: get_time() as f32 };

    let mut mesh_queue =
        GLOBAL_STATE.borrow_mut().mesh_queue.drain(..).collect_vec();

    mesh_queue.sort_by_key(|x| x.mesh.z_index);

    let mut draw_params = DrawParams {
        aspect_ratio: aspect_ratio(),
        config: &mut game_config_mut(),
        projection: main_camera().build_view_projection_matrix(),
        white_px: texture_path("1px"),
        clear_color,
        frame: frame_params,
        lights: LightingState::take_lights(),
        // sprite_queue,
        mesh_queue,
        particle_queue,
        egui: egui(),
    };

    // TODO: cleanup unwraps and stuff :)
    c.renderer.update(&mut draw_params);
    c.renderer.draw(draw_params);
    c.renderer.end_frame();
}

fn show_lighting_ui(_c: &mut EngineContext) {
    if game_config().dev.show_lighting_config {
        egui::Window::new("Lighting")
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 0.0))
            .show(egui(), |ui| {
                lighting_ui(&mut game_config_mut().lighting, ui);
            });

        // egui::Window::new("Post Processing").show(&self.egui(), |ui| {
        //     for i in 0..self.post_processing_effects.len() {
        //         ui.horizontal(|ui| {
        //             ui.add_enabled_ui(
        //                 i < self.post_processing_effects.len() - 1,
        //                 |ui| {
        //                     if ui.button("down").clicked() {
        //                         self.post_processing_effects.swap(i, i + 1);
        //                     }
        //                 },
        //             );
        //
        //             ui.add_enabled_ui(i > 0, |ui| {
        //                 if ui.button("up").clicked() {
        //                     self.post_processing_effects.swap(i - 1, i);
        //                 }
        //             });
        //
        //             let effect = &mut self.post_processing_effects[i];
        //             ui.checkbox(
        //                 &mut effect.enabled,
        //                 format!("{}: {}", i, effect.name),
        //             );
        //         });
        //     }
        // });
    }
}
