# Quick Sort

Al igual que el [Merge Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/03-merge-sort) es un **_algoritmo de ordenación recursivo_** muy util y poderoso. Es el algoritmo de ordenación usado por algunos motores de Javascript.

## ¿Cómo funciona?

Toma la lista a ordenar, define un **pivote**, que en este caso será el ultimo elemento de la lista, y la divide en dos listas más pequeñas: elementos **menores y mayores** al pivote, que se ordenan llamando al mismo [Quick Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/04-quick-sort) hasta llegar al caso base de una lista unitaria o de cero elementos. Finalmente, con estas dos listas ordenadas se unen siguiendo el orden: menores ordenados, pivote y mayores ordenados.

> Según la estrategia de elección del pivote existen variaciones para lograr un mayor rendimiento.

## Big O

### Complejidad Computacional

En su peor caso, que es cuando la lista esta ordenada, en cualquier sentido, el algoritmo tendra que comparar cada valor con el resto de los valores por lo que seria una complejidad **O(n^2)**.

En su caso promedio, donde la lista está ordenada aleatoriamente será mas probable conseguir listas mejor mezclada en ambos lados y por lo tanto evitar gran cantidad de comparaciones, lo que otorga una complejidad de **O(n log n)**.

### Complejidad Espacial

Debido a que estaremos creando nuevas listas con cada llamada recursiva hasta llegar al caso base, en el peor de los casos, tendremos tantas listas como elementos lo que nos da una complejidad de espacial **O(n)**.

En algunas versiones optimizadas que operan sobre los arrays, se consigue una complejidad de **O(log n)**.
