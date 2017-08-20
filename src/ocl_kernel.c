__kernel void render_waveform(
		__private uint const w,
		__private uint const h,
		__private float const scale,
		__private float const global_min,
		__private float const global_max,
		__global uint const* const offsets,
		__global float const* const mins,
		__global float const* const maxs,
		__global uchar* res)
{

	uint const x = get_global_id(0);
	uint begin_bin = offsets[x];
	uint end_bin = offsets[x+1]; 

	float min = mins[begin_bin];
	float max = maxs[begin_bin];
	for(uint i = begin_bin + 1; i < end_bin; i++){
		if(mins[i] < min) min = mins[i];
		if(maxs[i] > max) max = maxs[i];
	}

	uint begin_y = h/2 - floor(scale * (max - global_min));
	if(begin_y < 0) begin_y = 0;
	if(begin_y > h) begin_y = h;
	uint end_y = h/2 - floor(scale * (min - global_min));
	if(end_y < 0) end_y = 0;
	if(end_y > h) end_y = h;
	
	for(uint y = begin_y; y < end_y; y++){
		res[(x + y*w)*4 + 0] = 0;
		res[(x + y*w)*4 + 1] = 0;
		res[(x + y*w)*4 + 2] = 0;
		res[(x + y*w)*4 + 3] = 255;
	}
	
}
