from typer import Option

def register(app):
    @app.command()
    def new(
        directory : str = Option(..., "--dir", "-d", help="Ruta donde crearemos el proyecto")
        ):    
        print("Creando proyecto...")