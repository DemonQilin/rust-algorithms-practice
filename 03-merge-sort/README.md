# Merge Sort

Al ser un **_algoritmo de organización recursivo_** se basa en tomar una lista de elementos y dividirla en listas más pequeñas que organizar, y así sucesivamente hasta llegar a una lista de longitud 1 que por definición están organizadas.

## ¿Cómo funciona?

Tendremos dos funciones:

- **break**: rompe una lista en dos listas más pequeñas: Es recursiva.
- **merge**: toma dos listas ordenadas y retorna una lista ordenada. No recursiva.

## Big O

### Complejidad Computacional

A diferencia de los algoritmos [Bubble Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/01-bubble-sort) e [Insertion Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/02-insertion-sort) no debe comparar cada elemento con el resto de los demás elementos, lo que le permite ahorrarse una gran cantidad de comparaciones. Sin embargo, debido a la naturaleza de la tarea de organizar es necesario minimamente conocer cada elemento. Esto lo hace tener una complejidad computacional de **_O(n log n)_**.

> Es probablemente el algoritmo más estable porque todos sus casos son el peor caso, es decir, no existe diferencia entre casos porque todos se tratan igual.

### Complejidad Espacial

La cantidad máxima de memoria adicional necesaria en cualquier momento dado es proporcional a la cantidad total de elementos que estás ordenando, lo que da como resultado una complejidad espacial de **_O(n)_**, debido a que en el peor caso se tienen lista unitarias de cada elemento al mismo tiempo.

> En comparación con los otros algoritmos que eran constantes, en memoria representa un uso considerable si son listas grandes y poca memoria.
