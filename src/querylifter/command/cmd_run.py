def register(app):
    @app.command()
    def run():
        print("ejecutando scripts")