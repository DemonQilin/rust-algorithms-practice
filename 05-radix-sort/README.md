# Radix Sort

Es un algoritmo de ordenación **_que no usa comparación_** como criterio. Es decir, no itera a través de los elementos preguntando si un elemento es mayor a otro.

## ¿Cómo funciona?

Se basa en ordenar los números por su posiciones, es decir: unidades, decenas, centenas, y así sucesivamente, hasta completar la cantidad de posiciones del número más grande.

Esto se logra utilizando dos ciclos y **contenedores** para cada posición:

- **Externo**: itera sobre las posiciones de los números.
- **Interno**: itera sobre todos los elementos y los **encola** en los contenedores que pertenecen según el valor de la posición actual. Finalmente **desencola** los elementos de los contenedores en orden ascendente.

## Big O

### Computacional

Debido a que depende de dos parámetros muy diferentes: máximo de posiciones `k` y longitud de la lista `n`, su complejidad será **O(nk)**.

Si es mejor a **O(n log n)**, que es la complejidad más optima alcanzada por algoritmos de comparación como [Merge Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/03-merge-sort), **_dependerá_** del caso que se presente. Por ejemplo, en casos donde se tienen muchos números de longitudes variadas con una buena distribución puede tener un buen rendimiento.

## Espacial

Tiene una complejidad espacial de **O(n+k)** debido a que dependen de la cantidad máxima de posiciones y elementos. No es muy bueno en rendimiento espacial, por lo que sus aplicaciones cubren necesidades especificas.
