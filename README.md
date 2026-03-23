# CalWeek Finder

CalWeek Finder es una herramienta desarrollada en Rust diseñada para trabajar con fechas y semanas, específicamente en el formato de "calweek" utilizado en ciertos contextos laborales.

## ¿Qué es un "calweek"?

En este contexto, un "calweek" (calendar week) es un formato específico que consta de 4 dígitos:

- Los primeros dos dígitos representan el año (los últimos dos dígitos del año, por ejemplo, "23" para 2023).
- Los dos últimos dígitos representan el número de la semana en ese año, por ejemplo, "48" para la semana 48.

Por lo tanto, un "calweek" de "2348" se refiere a la semana 48 del año 2023.

## Características

- **Conversión de Fechas a CalWeek**: Convierte cualquier fecha en su correspondiente "calweek".
- **Formatos de Fecha Flexibles**: El programa admite los siguientes formatos de fecha:
  - `%Y-%m-%d`
  - `%d/%m/%Y`
  - `%m/%d/%Y`
  - `%d-%m-%Y`
  - `%d.%m.%Y`
  - Nota: fechas con `/` ambiguas (por ejemplo `01/02/2023`) se rechazan para evitar interpretar incorrectamente día/mes.

- **Obtener CalWeek Actual**: Proporciona el "calweek" de la fecha actual.
- **Determinar Semana Basada en CalWeek**: Dado un "calweek", encuentra el lunes y el domingo de esa semana específica.

## Uso

1. **Interfaz en Terminal (TUI)**: Si no introduces ningún argumento al ejecutar el programa, aparecerá una interfaz en la terminal que te permitirá elegir si quieres convertir de fecha a calweek o de calweek a fecha.
2. **Argumentos de Línea de Comandos (CLI)**:
   - **Obtener CalWeek Actual**: `calweek_finder today`
   - **Fecha a CalWeek**: `calweek_finder --date 2023-11-26`
   - **CalWeek a Fecha**: `calweek_finder --week 2348` (devuelve lunes y domingo en formato `dd.mm.yyyy`)
   - En un entorno no interactivo (sin TTY), ejecutar sin argumentos no abre el menú y devuelve error con instrucción de usar `--help`.
3. **Salida JSON**: Añade el flag `--json` a cualquiera de los comandos anteriores para obtener la salida en formato JSON.
   - Ejemplo: `calweek_finder --date 2023-11-26 --json` -> salida JSON con las claves `calweek`, `week_number` y `year`.

## Notas de formato y códigos de salida

- El formato `YYWW` interpreta `YY` como año ISO en el rango `2000-2099`.
- Las conversiones válidas devuelven código de salida `0`.
- Errores de validación (fecha o calweek inválida) devuelven código `1`.
- Uso inválido (por ejemplo, sin argumentos en no-interactivo) devuelve código `2`.
