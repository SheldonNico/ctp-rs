#! python3 autobind.py > src/wrapper_ext
import typing as t
import datetime, sys
from clang.cindex import Index, Config, CursorKind, Cursor

# ref to https://stackoverflow.com/questions/37336867/how-to-get-class-method-definitions-using-clang-python-bindings
TABS = " " * 4

def eprint(*args: t.Any, **kwargs: t.Any) -> None:  
    print(*args, file=sys.stderr, **kwargs)

def mainfun() -> None:
    index = Index.create()
    tu = index.parse("src/wrapper.hpp", args=["-Ishared/include", "-x", "c++", "--std=c++11"])
    for node in tu.cursor.get_children():
        if node.displayname in [
            "CThostFtdcTraderSpi",
            "CThostFtdcMdSpi"
        ]:
            eprint("found virtual node", node.kind, node.displayname, node.location.file.name, node.extent.start.offset, node.extent.end.offset)
            extract_virtual_class(node)
        if node.displayname in [
            "CThostFtdcTraderApi",
            "CThostFtdcMdApi",
        ]:
            eprint("found abc node", node.kind, node.displayname, node.location.file.name, node.extent.start.offset, node.extent.end.offset)
            extract_abc_class(node)

def extract_abc_class(node: Cursor) -> None:
    class_name = node.spelling
    rust_name = f"Rust_{class_name}"

    methods, methods_impl = [], []
    methods_impl.append(f"{rust_name}::{rust_name}({class_name} *inner) : m_inner(inner) {{}}")
    methods_impl.append(f"{rust_name}::~{rust_name}() {{ }}")
    for cc in node.get_children():
        if cc.kind == CursorKind.CXX_METHOD:
            if cc.is_virtual_method():
                assert not cc.type.is_function_variadic(), "not expect this to happen."

                argnames, argdecl = [], []
                for arg in cc.get_arguments():
                    argdecl.append(restore_orig_argument(arg))
                    argname = arg.spelling
                    argtype = arg.type.spelling

                    # Patch (std::string) ...
                    # std::string is not part of C abi
                    # use it as opaque type is Ok, but not convenient
                    if argtype in ["const std::string &"]:
                        argnames.append(f"{argname}.c_str()")
                    else:
                        argnames.append(argname)

                argdecl = ", ".join(argdecl)
                argnames = ", ".join(argnames)
                rtn_type = cc.type.get_result().spelling

                method_name = cc.spelling
                forward_body = f"return m_inner->{method_name}({argnames});"
                methods.append(f"{TABS*2}{rtn_type} {method_name}({argdecl});")
                methods_impl.append(f"{rtn_type} {rust_name}::{method_name}({argdecl}) {{ {forward_body} }}")
            else:
                pass
        else:
            pass

    methods = "\n".join(methods)
    methods_impl = "\n".join(methods_impl)
    destructor_override = " "

    print(f"""
class {rust_name} {{
    public:
        {class_name}* m_inner; 
        {rust_name}({class_name} *inner);
        ~{rust_name}(){destructor_override};
{methods}
}};
{methods_impl}""")

def extract_virtual_class(node: Cursor) -> None:
    class_name = node.spelling
    rust_name = f"Rust_{class_name}"
    forward_name = f"Rust_{class_name}_Trait"
    methods, methods_impl, extern_method_decl = [], [], []
    extern_method_decl.append(f"extern \"C\" void {forward_name}_Drop(void* m_rust);")
    methods_impl.append(f"{rust_name}::{rust_name}(void *rust) : m_rust(rust) {{}}")
    methods_impl.append(f"{rust_name}::~{rust_name}() {{ {forward_name}_Drop(m_rust); }}")
    destructor_virtual = False
    for cc in node.get_children():
        if cc.kind == CursorKind.CXX_METHOD:
            if cc.is_virtual_method():
                assert not cc.type.is_function_variadic(), "not expect this to happen."

                argnames, argdecl, argdecl_extern = ["m_rust"], [], ["void* m_rust"]
                for arg in cc.get_arguments():
                    argdecl.append(restore_orig_argument(arg))
                    argname = arg.spelling
                    argtype = arg.type.spelling

                    # Patch (std::string) ...
                    # std::string is not part of C abi
                    # use it as opaque type is Ok, but not convenient
                    if argtype in ["const std::string &"]:
                        argnames.append(f"{argname}.c_str()")
                        argdecl_extern.append(f"const char* {argname}")
                    else:
                        argnames.append(argname)
                        argdecl_extern.append(restore_orig_argument(arg))

                argdecl = ", ".join(argdecl)
                argdecl_extern = ", ".join(argdecl_extern)
                argnames = ", ".join(argnames)
                rtn_type = cc.type.get_result().spelling

                method_name = cc.spelling
                forward_method_name = f"{forward_name}_{method_name}"
                forward_body = f"return {forward_method_name}({argnames});"
                methods.append(f"{TABS*2}{rtn_type} {method_name}({argdecl}) override;")
                methods_impl.append(f"{rtn_type} {rust_name}::{method_name}({argdecl}) {{ {forward_body} }}")
                extern_method_decl.append(f"extern \"C\" {rtn_type} {forward_method_name}({argdecl_extern});")
        elif cc.kind == CursorKind.DESTRUCTOR:
            destructor_virtual = cc.is_virtual_method()
        elif cc.kind == CursorKind.CONSTRUCTOR:
            pass

    methods = "\n".join(methods)
    extern_method_decl = "\n".join(extern_method_decl)
    methods_impl = "\n".join(methods_impl)
    destructor_override = " override" if destructor_virtual else ""

    print(f"""
class Rust_{class_name} : {class_name} {{
    public:
        void* m_rust; 
        Rust_{class_name}(void *rust);
        ~Rust_{class_name}(){destructor_override};
{methods}
}};
{extern_method_decl}
{methods_impl}""")

def restore_orig_argument(node: Cursor) -> str:
    orig = ""
    start = None
    breaked = False
    for xx in node.get_tokens():
        if xx.spelling == "=": break # 跳过 arg1 = 12
        if start is not None: orig += " " * (xx.location.offset - start)
        orig += xx.spelling
        start = xx.location.offset + len(xx.spelling)

    if breaked: raise
    return orig

if __name__ == "__main__":
    print("// generated by python autobind.py @{:%Y-%m-%d}".format(datetime.datetime.now()))
    mainfun()
