use crate::__imports;

pub fn do_screen_fade_out(duration: i32) {
    __imports::natives_do_screen_fade_out(duration)
}

pub fn do_screen_fade_in(duration: i32) {
    __imports::natives_do_screen_fade_in(duration)
}

pub fn draw_marker(
    type_: i32,
    x: f32,
    y: f32,
    z: f32,
    dirX_: f32,
    dirY_: f32,
    dirZ_: f32,
    rotX_: f32,
    rotY_: f32,
    rotZ_: f32,
    scaleX_: f32,
    scaleY_: f32,
    scaleZ_: f32,
    red_: i32,
    green_: i32,
    blue_: i32,
    alpha_: i32,
    bobUpAndDown_: bool,
    faceCamera_: bool,
    p19_: i32,
    rotate_: bool,
    textureDict: &String,
    textureName: &String,
    drawOnEnts_: bool,
) {
    __imports::natives_draw_marker(
        type_,
        x,
        y,
        z,
        dirX_,
        dirY_,
        dirZ_,
        rotX_,
        rotY_,
        rotZ_,
        scaleX_,
        scaleY_,
        scaleZ_,
        red_,
        green_,
        blue_,
        alpha_,
        bobUpAndDown_,
        faceCamera_,
        p19_,
        rotate_,
        textureDict,
        textureName,
        drawOnEnts_,
    );
}
