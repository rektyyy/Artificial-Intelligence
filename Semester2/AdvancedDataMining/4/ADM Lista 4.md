---
title: ADM Lista 4
tags: [ADM]

---

# ADM Lista 4

## Zadanie 1

### Podpunkt 1
### Macierz sąsiedztwa ($A$)
$$A = \begin{pmatrix} 0 & 1 & 0 \\ 1 & 0 & 1 \\ 0 & 1 & 0 \end{pmatrix}$$

### Macierz stopni ($D$)
$$D = \begin{pmatrix} 1 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 1 \end{pmatrix}$$

### Laplasjan grafu ($L$)
$$L = \begin{pmatrix} 1 & -1 & 0 \\ -1 & 2 & -1 \\ 0 & -1 & 1 \end{pmatrix}$$


### Podpunkt 2
$$Lx = \begin{pmatrix} 1 & -1 & 0 \\ -1 & 2 & -1 \\ 0 & -1 & 1 \end{pmatrix} \begin{bmatrix} 1 \\ 0 \\ 0 \end{bmatrix} = \begin{bmatrix} 1 \\ -1 \\ 0 \end{bmatrix}$$

* Wierzchołki niezerowe: 1 oraz 2.
* Zależność: Wartość $(Lx)_i$ zależy od wartości w wierzchołku $i$ oraz jego bezpośrednich sąsiadów.
* Opis operacji: $Lx$ oblicza przejście sygnału między sąsiadami w pierwszym kroku propagacji.

### Podpunkt 3
$$L^2x = \begin{pmatrix} 1 & -1 & 0 \\ -1 & 2 & -1 \\ 0 & -1 & 1 \end{pmatrix} \begin{bmatrix} 1 \\ -1 \\ 0 \end{bmatrix} = \begin{bmatrix} 2 \\ -3 \\ 1 \end{bmatrix}$$

* Wierzchołek 3: Posiada teraz wartość niezerową (1).
* Zasięg: $L^2x$ zależy nie tylko od bezpośrednich sąsiadów, ale także od wierzchołków oddalonych o 2 kroki. 

### Podpunkt 4
Operacja $L^kx$ odpowiada procesowi propagacji informacji na grafie.

Jeśli wektor $x$ ma wartość niezerową w danym wierzchołku, to po $k$ aplikacjach macierzy $L$, informacja ta "rozleje się" na wszystkie węzły osiągalne w maksymalnie $k$ krokach (krawędziach). 

## Zadanie 2

### Podpunkt 1
Dla grafu z wagami krawędzi $w_{12}=1$ oraz $w_{23}=2$:

$$L = D - A = \begin{pmatrix} 1 & -1 & 0 \\ -1 & 3 & -2 \\ 0 & -2 & 2 \end{pmatrix}$$

### Podpunkt 2
$$Lx = \begin{bmatrix} 1 \\ -1 \\ 0 \end{bmatrix}$$
Silniejszy wpływ występuje między wierzchołkiem 2 a 3 ($w_{23}$=2 > $w_{12}$=1)

### Podpunkt 3
$$L^2x = \begin{pmatrix} 1 & -1 & 0 \\ -1 & 3 & -2 \\ 0 & -2 & 2 \end{pmatrix} \begin{bmatrix} 1 \\ -1 \\ 0 \end{bmatrix} = \begin{bmatrix} 2 \\ -4 \\ 2 \end{bmatrix}$$

W wierzchołku 3 pojawiła się wartość niezerowa (2). Oznacza to, że po dwóch operacjach macierzy $L$, sygnał przebył drogę o długości 2 krawędzi.

### Podpunkt 4

Wykorzystamy metodę bezpośredniej sumacji po krawędziach.

Z definicji $(Lx)_i = \sum_{j \sim i} w_{ij}(x_i - x_j)$. Formę kwadratową zapisujemy jako:
$$x^T L x = \sum_{i \in V} x_i (Lx)_i = \sum_{i \in V} \sum_{j \sim i} w_{ij} (x_i^2 - x_i x_j)$$

Ponieważ graf jest nieskierowany, każda krawędź $\{i, j\} \in E$ pojawia się w tej podwójnej sumie dokładnie dwa razy: raz jako składnik dla wierzchołka $i$ i raz dla wierzchołka $j$. Grupując te pary dla każdej krawędzi:

$$x^T L x = \sum_{\{i,j\} \in E} \left[ w_{ij}(x_i^2 - x_i x_j) + w_{ji}(x_j^2 - x_j x_i) \right]$$

Przyjmując symetrię wag ($w_{ij} = w_{ji}$):
$$x^T L x = \sum_{\{i,j\} \in E} w_{ij} \left( x_i^2 - 2x_i x_j + x_j^2 \right)$$

Stosując wzór skróconego mnożenia:
$$x^T L x = \sum_{(i,j) \in E} w_{ij} (x_i - x_j)^2$$


### Podpunkt 5

Wagi $w_{ij}$ pełnią rolę współczynników kary za różnicę wartości między sąsiednimi węzłami.

Gdy $w_{ij} \to \infty$, koszt różnicy $(x_i - x_j)$ w energii układu staje się ogromny. Aby zminimalizować $x^T L x$, wartości $x_i$ oraz $x_j$ muszą dążyć do równości.

# Zadanie 3

### Podpunkt 1

Z definicji macierzy Laplaciana $L = D - W$:
$$L\mathbf{1} = (D - W)\mathbf{1} = D\mathbf{1} - W\mathbf{1}$$

Rozpatrzmy $i$-tą składową tego wektora:
1. Dla macierzy diagonalnej $D$:
   $$(D\mathbf{1})_i = D_{ii} \cdot 1 = D_{ii}$$
   Z definicji macierzy stopni, $D_{ii}$ jest sumą wag wszystkich krawędzi wychodzących z wierzchołka $i$: $D_{ii} = \sum_j w_{ij}$.

2. Dla macierzy sąsiedztwa $W$:
   $$(W\mathbf{1})_i = \sum_j W_{ij} \cdot 1 = \sum_j w_{ij}$$

Podstawiając obie części:
$$(L\mathbf{1})_i = \sum_j w_{ij} - \sum_j w_{ij} = 0$$

Ponieważ każda składowa wektora wynikowego jest równa 0, otrzymujemy $L\mathbf{1} = \mathbf{0}$. Oznacza to, że wektor stały zawsze należy do jądra (kernela) macierzy Laplaciana.


### Podpunkt 2
Intuicja:
Wartość $(Lx)_i$ mówi nam, jak bardzo wartość w punkcie $i$ różni się od wartości u jego sąsiadów. 
