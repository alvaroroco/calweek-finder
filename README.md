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

- **Obtener CalWeek Actual**: Proporciona el "calweek" de la fecha actual.
- **Determinar Semana Basada en CalWeek**: Dado un "calweek", encuentra el lunes y el domingo de esa semana específica.

## Uso

1. **Interfaz en Terminal (TUI)**: Si no introduces ningún argumento al ejecutar el programa, aparecerá una interfaz en la terminal que te permitirá elegir si quieres convertir de fecha a calweek o de calweek a fecha.
2. **CalWeek Actual**: Si introduces "today", te devuelve el "calweek" actual.
3. **Fecha a CalWeek**: Si introduces una fecha válida en uno de los formatos admitidos, te devuelve el "calweek" correspondiente a ese día.
4. **Determinar Semana por CalWeek**: Si introduces un "calweek" válido, te devuelve el lunes y el domingo de esa semana específica.
