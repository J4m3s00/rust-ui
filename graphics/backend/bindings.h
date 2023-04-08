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

enum AppEventType
{
    AppEventType_None,
    AppEventType_Quit,
    AppEventType_KeyDown,
    AppEventType_KeyUp,
    AppEventType_MouseDown,
    AppEventType_MouseUp,
    AppEventType_MouseMove,
    AppEventType_MouseWheel,
    AppEventType_WindowResize,
};

typedef struct AppEvent
{
    enum AppEventType type;
    int key;
    int x;
    int y;
} AppEvent;

// initApp is borrowed.
EXPORT int c_start_application(const InitApp *initApp);
// Ownership of AppEvent is transferred to the caller.
// Update application and return an event.
// After that you can render things
EXPORT AppEvent *c_pre_update_application();

// Post rendering update of the application
EXPORT void c_post_update_application();
EXPORT void c_clean_up_editor();

EXPORT void c_draw_rect(float x, float y, float width, float height);
EXPORT void c_draw_circle(float x, float y, float radius);
EXPORT void c_draw_line(float x1, float y1, float x2, float y2);
EXPORT void c_draw_text(float x, float y, const char *text);