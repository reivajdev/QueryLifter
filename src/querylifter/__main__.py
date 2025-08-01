from querylifter.application.cli import register_cli

_app = register_cli()
if __name__ == "__main__":
    _app()