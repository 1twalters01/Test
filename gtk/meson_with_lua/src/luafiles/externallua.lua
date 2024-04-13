Var = "Lua printed this externally"
print("print out side of anything")

function Foo (x)
    print("print inside function")
    return Var.." "..x
end
