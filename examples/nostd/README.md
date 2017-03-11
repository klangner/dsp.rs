# Using library with embedded development 

In the case of embedded development it is important to be able to compile library with no-std settings 
(without standard library).

Currently it will not compile since dsp library requires Vec, which is defined in std.