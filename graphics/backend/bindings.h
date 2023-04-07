#if __cplusplus
    #define EXTERN extern "C"
#else
    #define EXTERN
#endif

#if defined(_WIN32) || defined(_WIN64)
    #if defined(INTERFACE_EXPORT)
        #define EXPORT __declspec(dllexport)
    #else
        #define EXPORT __declspec(dllimport)
    #endif
#endif

EXPORT int c_hello_world();
EXPORT int c_start_application();
