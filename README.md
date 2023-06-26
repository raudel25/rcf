# rcf

Este proyecto tiene como objetivo automatizar el proceso de solución de un problema de programación competitiva, por el momento solo disponible para la plataforma <a href="https://codeforces.com/">Codeforces</a>.

## Dependencias 

El proyecto se encuentra desarrollado en **Rust**, puede consultar las bibliotecas de las que depende en el archivo `Cargo.toml`, para instalar dichas dependencias puede usar el comando:

```
cargo build
```

## Ejecutando el Proyecto

Para ejecutar el proyecto debe contar en su sistema con el entorno de **Rust** instalado y ejecutar el siguiente comando:

```
cargo run <params>
```

Los parámetros se especificaran en el apartado de **Funcionalidades**.

### Funcionalidades

- **Clonar un Problema**: prepara el entorno para resolver un problema, incluyendo los casos de prueba de ejemplo y la plantilla de código especificada por el usuario, ejemplo:

```
cargo run problem <id_contest> <id_problem> <path> <language>
```

Los dos primeros parámetros son obligatorios y representan el **contest** y el **problem** específico, los restantes son para configurar el **path** donde vamos a clonar el problema y el **lenguaje** que vamos a utilizar.

- **Clonar un Concurso**: clona cada uno de los problemas de un **contest**, ejemplo:

```
cargo run contest <id_contest> <path> <language>
```

Los parámetros son exactamente los mismos que para la funcionalidad anterior pero aquí no se específica el `<id_problem>`.

- **Tester**: comprueba la solución del problema mediante los casos de prueba de ejemplo:

```
cargo run test <path>
```

El parámetro `<path>` no es obligatorio.

### Configuración

Puede agregar a la configuración los **lenguajes** de su preferencia, así como una plantilla de código para cada uno, puede generar la configuración por defecto mediante el comando:

```
cargo run config
```

Por defecto este se guarda en el **home** y está en formato **json**, ejemplo:

```json
{
    "default": "cpp",
    "languages": [
        {
            "name": "cpp",
            "compiler": "g++",
            "extension": ".cpp",
            "source": [],
            "executable": true
        },
        {
            "name": "python",
            "compiler": "python3",
            "extension": ".py",
            "source": [
                "print(\"hello world\")"
            ],
            "executable": false
        }
    ]
}
```

Los campos **compiler** y **extension**, representan el compilador se su **lenguaje** en su sistema y la extensión de dicho lenguaje, **source** representa la plantilla de código, mientras que el valor de **executable** depende de si su **lenguaje** necesita generar un ejecutable y luego ejecutar dicho ejecutables, por ejemplo **c** y **c++**.