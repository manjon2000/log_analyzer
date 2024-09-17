# Analizador de logs con procesamiento paralelo

### Requerimientos Funcionales

1. **Lectura de Logs**:
    - El sistema debe ser capaz de leer archivos de logs en formatos comunes.
    - Soporte para leer logs desde diferentes fuentes: archivos locales.

2. **Procesamiento Paralelo**:
    - El sistema debe distribuir el procesamiento de logs entre múltiples threads para aprovechar al máximo los recursos de la CPU.

3. **Filtrado de Logs**:
    - Posibilidad de aplicar filtros a los logs basados en criterios como rango de tiempo, niveles de log (info, warning, error), palabras clave.

4. **Agregación de Datos**:
    - Capacidad para agregar datos de los logs, como contar el número de errores, agrupar por tipos de eventos, etc.

5. **Alertas en Tiempo Real**:
    - Implementar un sistema de alertas que notifique al usuario en tiempo real cuando ocurran ciertos eventos críticos en los logs.

6. **Salida de Resultados**:
    - Los resultados deben poder mostrarse en tiempo real en la consola.
    - Integración opcional con sistemas de visualización de datos o dashboards.