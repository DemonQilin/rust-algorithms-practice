# Binary Search

Basicamente existen dos tipos de algoritmos de busqueda: lineal y binario. Donde el lineal se refiere a recorrer el array de elementos comparando cada elemento con el objetivo hasta que encontrarlo, en su peor caso deberá recorrer el array completo.

## ¿Cómo funciona?

La premisa más importante para que el algoritmo sea aplicable es que el array de elementos **_debe estar organizado_**.
Una vez se garantiza el orden, el algoritmo comienza buscando en el **_elemento de la mitad_** y comparando este con el objetivo, si no coincide, entonces según **si es mayor o menor buscara en el elemento del medio de la mitad derecha o izquierda**, respectivamente, y si este no es, hará este proceso sucesivamente hasta encontrar el elemento.

## Big O

### Complejidad Computacional

Debido a que este algoritmo no recorre todos los elementos su complejidad es de **O(log n)** lo cual quiere decir que su crecimiento en tiempo de ejecución crece muy lentamente respecto al crecimiento de la cantidad de elementos.

> Es muy efectivo siempre que se tenga el array organizado.

### Complejidad Espacial

Debido a que no instancia ningún tipo de estructura adicional su complejidad espacial es constante, es decir, **O(1)**.
