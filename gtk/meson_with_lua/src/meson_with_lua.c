#include <gtk-4.0/gtk/gtk.h>
#include <lua.h>
#include <lualib.h>
#include <lauxlib.h>

#include "cfiles/printc.c"
#include "luafiles/inlinelua.c"


static void print_external_lua(void) {
    lua_State *L = luaL_newstate();

    luaL_openlibs(L);
    // luaL_dofile(L, "luafiles/varlua.lua");
    luaL_dofile(L, "../src/luafiles/externallua.lua");

    lua_getglobal(L, "Var");
    const char* var = lua_tostring(L, -1);
    printf("test: %s\n", var);
    // lua_pop(L, 1);
    // lua_close(L);

    lua_getglobal(L, "Foo");
    lua_pushstring(L, "test");
    lua_pcall(L, 1, 1, 0);
    const char* result = lua_tostring(L, -1);
    printf("x: %s\n", result);

    lua_close(L);
}

// static void activate(GtkApplication *app, gpointer user_data) {
static void activate(GtkApplication *app) {
    GtkWidget *window;
    GtkWidget *grid;
    GtkWidget *button1;
    GtkWidget *button2;
    GtkWidget *button3;

    window = gtk_application_window_new(app);
    gtk_window_set_title(GTK_WINDOW(window), "Summarize");
    gtk_window_set_default_size(GTK_WINDOW(window), 500, 200);

    grid = gtk_grid_new();
    gtk_grid_set_column_spacing(GTK_GRID(grid), 50);
    // gtk_grid_set_row_spacing(GTK_GRID(grid), 6);

    button1 = gtk_button_new_with_label("C Button");
    g_signal_connect(button1, "clicked", G_CALLBACK(print_c), NULL);
    button2 = gtk_button_new_with_label("Lua Button (inline)");
    g_signal_connect(button2, "clicked", G_CALLBACK(print_inline_lua), NULL);
    button3 = gtk_button_new_with_label("Lua Button (inline)");
    g_signal_connect(button3, "clicked", G_CALLBACK(print_external_lua), NULL);

    gtk_grid_attach(GTK_GRID(grid), button1, 0, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button2, 1, 0, 1, 1);
    gtk_grid_attach(GTK_GRID(grid), button3, 2, 0, 1, 1);

    gtk_window_set_child(GTK_WINDOW(window), grid);

    gtk_window_present(GTK_WINDOW(window));
}

int main(int argc, char *argv[]) {
    GtkApplication *app;
    int status;

    app = gtk_application_new("uk.summarize.app", G_APPLICATION_FLAGS_NONE);
    g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
    status = g_application_run(G_APPLICATION(app), argc, argv);
    g_object_unref(app);

    return status;
}


