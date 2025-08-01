from importlib import import_module
from pkgutil import iter_modules
from typer import Typer
from querylifter import command

def register_cli() -> Typer:
    app = Typer()

    for finder, module_name, ispkg in iter_modules(command.__path__):
        # Ignorar módulos "privados" o carpetas
        if module_name.startswith("_") or ispkg:
            continue
        try:
            module = import_module(f"{command.__name__}.{module_name}")
            if hasattr(module, "register"):
                module.register(app)
        except Exception as e:
            # Puedes loggear esto con logging si lo prefieres
            print(f"[ERROR] No se pudo registrar el módulo '{module_name}': {e}")

    return app