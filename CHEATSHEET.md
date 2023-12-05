## Mouse Cursor

- Hide mouse cursor: `c.renderer.window().set_cursor_visible(false);`

## Camera

- Center camera: `main_camera_mut().center = Vec2::from([x, y]);`
- Zoom camera: `main_camera_mut().zoom = f32;`
- Resolution: `GameConfig { resolution: ResolutionConfig::Physical(u32, u32), minimum_resolution: ResolutionConfig::Physical(u32, u32),
..config }`

## hecs

For loops of queries acts as System within ECS
- Query entities: `for (entity), (value, value)) in world().query::<&Value>().iter()`

 

