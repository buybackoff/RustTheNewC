using System;
using System.Runtime.InteropServices;
using System.Text;

namespace RustTheNewC
{
    internal static class Program
    {
        private const string NativeLibraryName = "corelib";

        [DllImport(NativeLibraryName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr native_versions_get();

        [DllImport(NativeLibraryName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void native_versions_free(IntPtr cString);

        private static void Main(string[] args)
        {
            var cStringPtr = native_versions_get();
            try
            {
                Console.WriteLine(PtrToStringUtf8(cStringPtr));
            }
            finally
            {
                native_versions_free(cStringPtr);
            }
        }

        private static string PtrToStringUtf8(IntPtr ptr)
        {
            if (ptr == IntPtr.Zero)
            {
                return null;
            }

            var i = 0;
            while (Marshal.ReadByte(ptr, i) != 0)
            {
                i++;
            }

            var bytes = new byte[i];
            Marshal.Copy(ptr, bytes, 0, i);

            return Encoding.UTF8.GetString(bytes, 0, i);
        }
    }
}
