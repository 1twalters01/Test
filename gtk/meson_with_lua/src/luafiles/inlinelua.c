#include <lua.h>
#include <lauxlib.h>

void print_inline_lua(void);

void print_inline_lua(void) {
    lua_State *L = luaL_newstate();

    // load and execute a string
    luaL_dostring(L, "function concatenation (x,y) return x ..y end");
    // if (luaL_dostring(L, "function concatenation (x,y) return x ..y end")) {
    //     lua_close(L);
    // }

    // push value of global "concatenation" (the function defined above) to the stack,
    lua_getglobal(L, "concatenation");
    lua_pushstring(L, "Lua printed this");
    lua_pushstring(L, " (inline)");

    lua_call(L, 2, 1); // call a function with two arguments and one return value
    printf("%s\n", lua_tostring(L, -1)); // print integer value of item at stack top
    lua_pop(L, 1); // return stack to original state
    lua_close(L); // close Lua state
}

