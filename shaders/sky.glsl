const float PI = 3.141592;

float sun_phase() {
    //return fract(anim * 0.1) * PI * 2.; 
    return fract(anim * 0.005) * PI * 2.; 
}

vec3 sun_pos() {
    //return vec3(1., 0.1, 1.);
    float a = sun_phase();
    return vec3(0., cos(a), sin(a));
}

vec3 sky_color(vec3 dir) {
    vec3 sun = sun_pos();
    vec3 day = mix(
        vec3(0.835,0.476,0.831), 
    	vec3(0.000,0.468,1.000), 
        clamp(dir.y * 3., 0., 1.)
    );
    vec3 twilight = mix(
        vec3(0.610,0.304,0.113), 
    	vec3(0.000,0.360,0.770), 
        clamp(dir.y * 2. + -0.040, 0., 1.)
    );
    vec3 night = vec3(0., 0.005, 0.01);
    float v = clamp(cos(sun_phase()) + 1., 0., 1.);
    return mix(
        night,
        mix(twilight, day, v),
        v
    );
}

vec3 sky(vec3 dir) {
    vec3 sunp = sun_pos();
    if (length(cross(dir, sunp)) < 0.1 && dot(dir, sunp) > 0.) {
        return vec3(1.);
    } else {
        return sky_color(dir);
    }
}
