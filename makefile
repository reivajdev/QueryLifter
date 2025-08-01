# Nombre del ejecutable final
NAME=querylifter

# Ruta al archivo principal
ENTRY=src/querylifter/__main__.py

# Carpeta de salida
OUTDIR=release

.PHONY: build clean run

## 🔨 Compila el proyecto con pyinstaller
build:
	shiv -c qlf -o dist/querylifter.pyz -r requirements.txt .
	
## 🧹 Limpia archivos temporales
clean:
	rm -rf build __pycache__ *.spec $(OUTDIR)

## 🚀 Ejecuta el ejecutable generado
run: build
	./$(OUTDIR)/$(NAME).exe --help
