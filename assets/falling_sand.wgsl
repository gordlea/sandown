float rand_n(float r, int n)
{
    for (int i = 0; i < n; ++i)
    {
        r = fract(r * 100.0);
    }
    
    return r;
}

void mainImage(out vec4 frag_color, in vec2 frag_coord)
{
    ivec2 coord = ivec2(frag_coord / scale);
    Particle self = particle_at(coord);
    
    switch (self.type)
    {
        case SAND:
        {
            frag_color = vec4(
                mix(0.9, 1.0, rand_n(self.shade, 1)), 
                mix(0.75, 0.8, rand_n(self.shade, 2)),
                mix(0.5, 0.6, rand_n(self.shade, 3)), 1.0) * mix(0.7, 1.0, self.shade);
            return;
        } break;
        
        case WALL:
        {
            frag_color = vec4(
                mix(0.3, 0.4, rand_n(self.shade, 1)), 
                mix(0.3, 0.4, rand_n(self.shade, 2)),
                mix(0.3, 0.4, rand_n(self.shade, 3)), 1.0) * mix(0.7, 1.0, self.shade);
            return;
        } break;
        
        case AIR:
        {
            frag_color = vec4(0.6, 0.8, 1.0, 1.0);
            return;
        }
    }
}