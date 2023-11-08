# Bubble Sort

Dentro de los algoritmos de ordenamiento suele ser el primero que se enseña porque es el que mejor se adapta al modelo de pensamiento humano para ordenar números.

## ¿Cómo funciona?

Compara dos elementos que están uno al lado del otro y si están desordenados los intercambia, y lo hace con el siguiente elemento del array hasta el final, una vez finalizado el recorrido verifica si hubo algún intercambio y si lo hubo recorre nuevamente ejecutando la comparación hasta que no se presente intercambio.

> Se llama buble sort porque los elemento se mueven en una dirección, normalmente, los elementos mayores se mueven hacia el fina. Así, después de la primera iteración, el elemento mayor estará al final, luego en la segunda iteración el segundo mayor estará penúltimo, y así sucesivamente.

## Big O

### Complejidad computacional

En el peor de los casos este algoritmo tiene **_una complejidad O(n<sup>2</sup>)_** porque tienes dos ciclos: el que analiza si hubo intercambios y el que itera sobre el array.

Puede optimizarse restando comparaciones a medida que se hacen mas recorridos debido a que los últimos elementos movidos ya se encuentran ordenados y no es necesario comparar entre ellos, esto solo reduce el coeficiente de complejidad, que dependiendo del caso podría ser importante o no.

### Complejidad Espacial

La aplicación usual del algoritmo modifica el array original sin crear nada más en memoria, por lo que tiene una **_complejidad espacial O(1)_**.

## Otros aspectos

- Es un algoritmo **_destructivo_** porque opera sobre el array de entrada.
- Es un algoritmo de **_ordenación estable_**, lo cual significa que dos elementos iguales permanecen en el mismo orden en el que fueron ingresados.
