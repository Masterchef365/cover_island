vec3 sky_color(vec3 dir) {
    return mix(
        vec3(0.835,0.476,0.831), 
    	vec3(0.000,0.468,1.000), 
        (dir.y + -0.408) / 0.464
    );
}

vec3 sun_pos() {
    return vec3(1., 1., 1.);
}

vec3 sky(vec3 dir) {
    if (length(cross(dir, sun_pos())) < 0.1) {
        return vec3(1.);
    } else {
        return sky_color(dir);
    }
}
