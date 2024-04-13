#include <gtk-4.0/gtk/gtk.h>

void print_c(GtkWidget *widget, gpointer data);

void print_c(GtkWidget *widget, gpointer data) {
    printf("C printed this\n");
}

