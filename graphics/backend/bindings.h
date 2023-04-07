#if __cplusplus
#define EXTERN extern "C"
#else
#define EXTERN extern
#endif

#if defined(_WIN32) || defined(_WIN64)
#if defined(INTERFACE_EXPORT)
#define EXPORT EXTERN __declspec(dllexport)
#else
#define EXPORT EXTERN __declspec(dllimport)
#endif
#else
#define EXPORT EXTERN
#endif

typedef struct InitApp
{
    const char *title;
} InitApp;

EXPORT int c_start_application(const InitApp *initApp);
