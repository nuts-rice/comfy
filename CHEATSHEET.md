## Mouse Cursor

- Hide mouse cursor: `c.renderer.window().set_cursor_visible(false);`

## Camera

- Center camera: `main_camera_mut().center = Vec2::from([x, y]);`
- Zoom camera: `main_camera_mut().zoom = f32;`
- Resolution: `GameConfig { resolution: ResolutionConfig::Physical(u32, u32), minimum_resolution: ResolutionConfig::Physical(u32, u32),
..config }`

## Assets

- Load Sprites: `c.load_texture_from_bytes("texture", 
   include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"),
   "/assets/texture.png"
   )),
   );
   `

- Draw sprites with params: `draw_sprite_ex(texture_id("texture"), 
                             Vec2::ZERO,
                             WHITE,
                             0,
                             DrawTextureParams {
                             dest_size: Some(vec2(f32, f32).as_world_size()),
                             ..Default::default())
                             })`

## Shaders

- Get ShaderMap: `c.renderer.shaders.borrow_mut`
 

## hecs

For loops of queries acts as System within ECS
- Query entities: `for (entity), (value, value)) in world().query::<&Value>().iter()`

 

