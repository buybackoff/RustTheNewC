/* automatically generated by rust-bindgen */

extern "C" {
    #[doc = "Compress a block of data in the `src` buffer and returns the size of"]
    #[doc = "the compressed block.  The size of `src` buffer is specified by"]
    #[doc = "`nbytes`.  There is not a minimum for `src` buffer size (`nbytes`)."]
    #[doc = ""]
    #[doc = "`clevel` is the desired compression level and must be a number"]
    #[doc = "between 0 (no compression) and 9 (maximum compression)."]
    #[doc = ""]
    #[doc = "`doshuffle` specifies whether the shuffle compression filters"]
    #[doc = "should be applied or not.  BLOSC_NOSHUFFLE means not applying it,"]
    #[doc = "BLOSC_SHUFFLE means applying it at a byte level and BLOSC_BITSHUFFLE"]
    #[doc = "at a bit level (slower but may achieve better entropy alignment)."]
    #[doc = ""]
    #[doc = "`typesize` is the number of bytes for the atomic type in binary"]
    #[doc = "`src` buffer.  This is mainly useful for the shuffle filters."]
    #[doc = "For implementation reasons, only a 1 < `typesize` < 256 will allow the"]
    #[doc = "shuffle filter to work.  When `typesize` is not in this range, shuffle"]
    #[doc = "will be silently disabled."]
    #[doc = ""]
    #[doc = "The `dest` buffer must have at least the size of `destsize`.  Blosc"]
    #[doc = "guarantees that if you set `destsize` to, at least,"]
    #[doc = "(`nbytes` + BLOSC_MAX_OVERHEAD), the compression will always succeed."]
    #[doc = "The `src` buffer and the `dest` buffer can not overlap."]
    #[doc = ""]
    #[doc = "Compression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer beyond what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If `src` buffer cannot be compressed into `destsize`, the return"]
    #[doc = "value is zero and you should discard the contents of the `dest`"]
    #[doc = "buffer."]
    #[doc = ""]
    #[doc = "A negative return value means that an internal error happened.  This"]
    #[doc = "should never happen.  If you see this, please report it back"]
    #[doc = "together with the buffer data causing this and compression settings."]
    #[doc = ""]
    #[doc = "Environment variables"]
    #[doc = "---------------------"]
    #[doc = ""]
    #[doc = "blosc_compress() honors different environment variables to control"]
    #[doc = "internal parameters without the need of doing that programatically."]
    #[doc = "Here are the ones supported:"]
    #[doc = ""]
    #[doc = "BLOSC_CLEVEL=(INTEGER): This will overwrite the `clevel` parameter"]
    #[doc = "before the compression process starts."]
    #[doc = ""]
    #[doc = "BLOSC_SHUFFLE=[NOSHUFFLE | SHUFFLE | BITSHUFFLE]: This will"]
    #[doc = "overwrite the `doshuffle` parameter before the compression process"]
    #[doc = "starts."]
    #[doc = ""]
    #[doc = "BLOSC_TYPESIZE=(INTEGER): This will overwrite the `typesize`"]
    #[doc = "parameter before the compression process starts."]
    #[doc = ""]
    #[doc = "BLOSC_COMPRESSOR=[BLOSCLZ | LZ4 | LZ4HC | SNAPPY | ZLIB]: This will"]
    #[doc = "call blosc_set_compressor(BLOSC_COMPRESSOR) before the compression"]
    #[doc = "process starts."]
    #[doc = ""]
    #[doc = "BLOSC_NTHREADS=(INTEGER): This will call"]
    #[doc = "blosc_set_nthreads(BLOSC_NTHREADS) before the compression process"]
    #[doc = "starts."]
    #[doc = ""]
    #[doc = "BLOSC_BLOCKSIZE=(INTEGER): This will call"]
    #[doc = "blosc_set_blocksize(BLOSC_BLOCKSIZE) before the compression process"]
    #[doc = "starts.  *NOTE:* The blocksize is a critical parameter with"]
    #[doc = "important restrictions in the allowed values, so use this with care."]
    #[doc = ""]
    #[doc = "BLOSC_NOLOCK=(ANY VALUE): This will call blosc_compress_ctx() under"]
    #[doc = "the hood, with the `compressor`, `blocksize` and"]
    #[doc = "`numinternalthreads` parameters set to the same as the last calls to"]
    #[doc = "blosc_set_compressor(), blosc_set_blocksize() and"]
    #[doc = "blosc_set_nthreads().  BLOSC_CLEVEL, BLOSC_SHUFFLE, BLOSC_TYPESIZE"]
    #[doc = "environment vars will also be honored."]
    #[doc = ""]
    #[doc = "BLOSC_SPLITMODE=[ FORWARD_COMPAT | AUTO | ALWAYS | NEVER ]:"]
    #[doc = "This will call blosc_set_splitmode() with the different supported values."]
    #[doc = "See blosc_set_splitmode() docstrings for more info on each mode."]
    #[doc = ""]
    pub fn blosc_compress(
        clevel: libc::c_int,
        doshuffle: libc::c_int,
        typesize: usize,
        nbytes: usize,
        src: *const libc::c_void,
        dest: *mut libc::c_void,
        destsize: usize,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Context interface to blosc compression. This does not require a call"]
    #[doc = "to blosc_init() and can be called from multithreaded applications"]
    #[doc = "without the global lock being used, so allowing Blosc be executed"]
    #[doc = "simultaneously in those scenarios."]
    #[doc = ""]
    #[doc = "It uses the same parameters than the blosc_compress() function plus:"]
    #[doc = ""]
    #[doc = "`compressor`: the string representing the type of compressor to use."]
    #[doc = ""]
    #[doc = "`blocksize`: the requested size of the compressed blocks.  If 0, an"]
    #[doc = "automatic blocksize will be used."]
    #[doc = ""]
    #[doc = "`numinternalthreads`: the number of threads to use internally."]
    #[doc = ""]
    #[doc = "A negative return value means that an internal error happened.  This"]
    #[doc = "should never happen.  If you see this, please report it back"]
    #[doc = "together with the buffer data causing this and compression settings."]
    pub fn blosc_compress_ctx(
        clevel: libc::c_int,
        doshuffle: libc::c_int,
        typesize: usize,
        nbytes: usize,
        src: *const libc::c_void,
        dest: *mut libc::c_void,
        destsize: usize,
        compressor: *const libc::c_char,
        blocksize: usize,
        numinternalthreads: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Decompress a block of compressed data in `src`, put the result in"]
    #[doc = "`dest` and returns the size of the decompressed block."]
    #[doc = ""]
    #[doc = "The `src` buffer and the `dest` buffer can not overlap."]
    #[doc = ""]
    #[doc = "Decompression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer beyond what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If an error occurs, e.g. the compressed data is corrupted or the"]
    #[doc = "output buffer is not large enough, then 0 (zero) or a negative value"]
    #[doc = "will be returned instead."]
    #[doc = ""]
    #[doc = "Environment variables"]
    #[doc = "---------------------"]
    #[doc = ""]
    #[doc = "blosc_decompress() honors different environment variables to control"]
    #[doc = "internal parameters without the need of doing that programatically."]
    #[doc = "Here are the ones supported:"]
    #[doc = ""]
    #[doc = "BLOSC_NTHREADS=(INTEGER): This will call"]
    #[doc = "blosc_set_nthreads(BLOSC_NTHREADS) before the proper decompression"]
    #[doc = "process starts."]
    #[doc = ""]
    #[doc = "BLOSC_NOLOCK=(ANY VALUE): This will call blosc_decompress_ctx()"]
    #[doc = "under the hood, with the `numinternalthreads` parameter set to the"]
    #[doc = "same value as the last call to blosc_set_nthreads()."]
    pub fn blosc_decompress(
        src: *const libc::c_void,
        dest: *mut libc::c_void,
        destsize: usize,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Context interface to blosc decompression. This does not require a"]
    #[doc = "call to blosc_init() and can be called from multithreaded"]
    #[doc = "applications without the global lock being used, so allowing Blosc"]
    #[doc = "be executed simultaneously in those scenarios."]
    #[doc = ""]
    #[doc = "It uses the same parameters than the blosc_decompress() function plus:"]
    #[doc = ""]
    #[doc = "`numinternalthreads`: number of threads to use internally."]
    #[doc = ""]
    #[doc = "Decompression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer more than what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If an error occurs, e.g. the compressed data is corrupted or the"]
    #[doc = "output buffer is not large enough, then 0 (zero) or a negative value"]
    #[doc = "will be returned instead."]
    pub fn blosc_decompress_ctx(
        src: *const libc::c_void,
        dest: *mut libc::c_void,
        destsize: usize,
        numinternalthreads: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Returns the current number of threads that are used for"]
    #[doc = "compression/decompression."]
    pub fn blosc_get_nthreads() -> libc::c_int;
}
extern "C" {
    #[doc = "Initialize a pool of threads for compression/decompression.  If"]
    #[doc = "`nthreads` is 1, then the serial version is chosen and a possible"]
    #[doc = "previous existing pool is ended.  If this is not called, `nthreads`"]
    #[doc = "is set to 1 internally."]
    #[doc = ""]
    #[doc = "Returns the previous number of threads."]
    pub fn blosc_set_nthreads(nthreads: libc::c_int) -> libc::c_int;
}
extern "C" {
    #[doc = "Returns the current compressor that is being used for compression."]
    pub fn blosc_get_compressor() -> *const libc::c_char;
}
extern "C" {
    #[doc = "Select the compressor to be used.  The supported ones are \"blosclz\","]
    #[doc = "\"lz4\", \"lz4hc\", \"snappy\", \"zlib\" and \"ztsd\".  If this function is not"]
    #[doc = "called, then \"blosclz\" will be used by default."]
    #[doc = ""]
    #[doc = "In case the compressor is not recognized, or there is not support"]
    #[doc = "for it in this build, it returns a -1.  Else it returns the code for"]
    #[doc = "the compressor (>=0)."]
    pub fn blosc_set_compressor(compname: *const libc::c_char) -> libc::c_int;
}
extern "C" {
    #[doc = "Get a list of compressors supported in the current build.  The"]
    #[doc = "returned value is a string with a concatenation of \"blosclz\", \"lz4\","]
    #[doc = "\"lz4hc\", \"snappy\", \"zlib\" or \"zstd \"separated by commas, depending"]
    #[doc = "on which ones are present in the build."]
    #[doc = ""]
    #[doc = "This function does not leak, so you should not free() the returned"]
    #[doc = "list."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_list_compressors() -> *const libc::c_char;
}
extern "C" {
    #[doc = "Return the version of the C-Blosc library in string format."]
    #[doc = ""]
    #[doc = "Useful for dynamic libraries."]
    pub fn blosc_get_version_string() -> *const libc::c_char;
}
extern "C" {
    #[doc = "Return information about a compressed buffer, namely the internal"]
    #[doc = "Blosc format version (`version`) and the format for the internal"]
    #[doc = "compressor used (`compversion`)."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_cbuffer_versions(
        cbuffer: *const libc::c_void,
        version: *mut libc::c_int,
        compversion: *mut libc::c_int,
    );
}
extern "C" {
    pub fn compress_lz4(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_lz4(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn compress_zstd(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_zstd(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn compress_zlib(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_zlib(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn compress_deflate(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_deflate(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn compress_gzip(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_gzip(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    pub fn compress_noop(
        input: *const libc::c_char,
        input_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
        clevel: libc::c_int,
    ) -> libc::c_int;
}
extern "C" {
    pub fn decompress_noop(
        input: *const libc::c_char,
        compressed_length: usize,
        output: *mut libc::c_char,
        maxout: usize,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Primary shuffle and bitshuffle routines."]
    #[doc = "This function dynamically dispatches to the appropriate hardware-accelerated"]
    #[doc = "routine based on the host processor's architecture. If the host processor is"]
    #[doc = "not supported by any of the hardware-accelerated routines, the generic"]
    #[doc = "(non-accelerated) implementation is used instead."]
    #[doc = "Consumers should almost always prefer to call this routine instead of directly"]
    #[doc = "calling the hardware-accelerated routines because this method is both cross-"]
    #[doc = "platform and future-proof."]
    pub fn shuffle(
        bytesoftype: usize,
        blocksize: usize,
        _src: *const libc::c_char,
        _dest: *const libc::c_char,
    );
}
extern "C" {
    pub fn bitshuffle(
        bytesoftype: usize,
        blocksize: usize,
        _src: *const libc::c_char,
        _dest: *const libc::c_char,
        _tmp: *const libc::c_char,
    ) -> libc::c_int;
}
extern "C" {
    #[doc = "Primary unshuffle and bitunshuffle routine."]
    #[doc = "This function dynamically dispatches to the appropriate hardware-accelerated"]
    #[doc = "routine based on the host processor's architecture. If the host processor is"]
    #[doc = "not supported by any of the hardware-accelerated routines, the generic"]
    #[doc = "(non-accelerated) implementation is used instead."]
    #[doc = "Consumers should almost always prefer to call this routine instead of directly"]
    #[doc = "calling the hardware-accelerated routines because this method is both cross-"]
    #[doc = "platform and future-proof."]
    pub fn unshuffle(
        bytesoftype: usize,
        blocksize: usize,
        _src: *const libc::c_char,
        _dest: *const libc::c_char,
    );
}
extern "C" {
    pub fn bitunshuffle(
        bytesoftype: usize,
        blocksize: usize,
        _src: *const libc::c_char,
        _dest: *const libc::c_char,
        _tmp: *const libc::c_char,
    ) -> libc::c_int;
}
