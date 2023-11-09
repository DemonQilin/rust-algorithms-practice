# Insertion Sort

Es un algoritmo de ordenación un poco más usado que el [Bubble Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/01-bubble-sort), normalmente es educativo, y en comparación tiene un coeficiente de complejidad menor.

## ¿Cómo funciona?

Se asume que la primera parte esta ordenada y la segunda desordenada, así se toma cada elemento siguiente a la lista ordenada y se compara con el último de la ordenada:

- Si es mayor se deja a la derecha y este será el ultimo elemento de la lista ordenada.
- Si es menor se ubica a la izquierda del último elemento y se compara con que el siguiente ahora tiene a la izquierda.

Se repite el proceso hasta llegar al ultimo elemento del array.

La primera vez el array ordenado es un array compuesto solo por el primer elemento del array a ordenar.

## Big O

## Complejidad computacional

Al igual que el [Bubble Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/01-bubble-sort) en sus escenarios promedio y peor tiene una **_complejidad de O(n<sup>2</sup>)_** debido a que tiene dos ciclos: el que recorre el array elemento por elemento, y el que compara cada elemento con el array ordenado.

> 🔥 Es muy recomendable cuando se tiene un array pequeño o muy cercano a estar ordenado.

## Complejidad Espacial

Al igual que [Bubble Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/01-bubble-sort) opera sobre el array por lo que no necesita crear items adicionales en memoria, es decir, **_complejidad de O(1)_**.

## Otros aspectos

- Es un algoritmo **_destructivo_** porque opera sobre el array de entrada.
- Dependiendo de como se programe podria ser un algoritmo de **_ordenación estable_**.

> En algunos casos de uso se puede tener **_combinado con algoritmos_** que hacia el caso promedio son mas eficientes, es decir, iniciar con este pero de verse superado un cierto número de iteraciones pasar a algoritmos mas eficientes como [Merge Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/03-merge-sort) o [Quick Sort](https://github.com/DemonQilin/rust-algorithms-practice/tree/main/04-quick-sort).
