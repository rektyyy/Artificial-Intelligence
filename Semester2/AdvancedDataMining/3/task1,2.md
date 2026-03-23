---
title: ADM Lista 2
tags: [ADM]

---

# ADM Lista 2

## Zadanie 1

### Podpunkt a)
$z_i$ - i-ty element rzedu z 

$$ e^x > 0 => \frac{e^{z_i}}{\sum_i e^{z_i}} > 0$$ 

### Podpunkt b)

$$\sum_i z_i => \sum_i softmax(z_i) => \frac{\sum_i e^{z_i}}{\sum_i e^{z_i}} = 1$$

### Podpunkt c)

$A_{ij}$ - jak duzo uwagi/atencji poswiecamy tokenowi j podczas przetwarzania tokenu i. 

P(skupienie na token j | token i)

## Zadanie 2

### Podpunkt a)

$PE(p, 2i) = sin(\frac{p}{10000^{\frac{2i}{d}}})$
$PE(p, 2i + 1) = cos(\frac{p}{10000^{\frac{2i}{d}}})$

$$(sin(\frac{p}{10000^{\frac{2i}{d}}}), cos(\frac{p}{10000^{\frac{2i}{d}}}))$$

$PE(q, 2i) = sin(\frac{q}{10000^{\frac{2i}{d}}})$
$PE(q, 2i + 1) = cos(\frac{q}{10000^{\frac{2i}{d}}})$

$$(sin(\frac{q}{10000^{\frac{2i}{d}}}), cos(\frac{q}{10000^{\frac{2i}{d}}}))$$

### Podpunkt b)
$M = 10000^{2i/d}$

$sin(\alpha + \beta) = sin \alpha cos \beta + cos \alpha sin \beta$

$cos(\alpha + \beta) = cos \alpha cos \beta - sin \alpha sin \beta$

$$PE(p, 2i) = sin(p/M) = sin(\frac{q + \Delta}{M}) = sin(q/M)*cos(\Delta/M) + cos(q/M)*sin(\Delta/M)$$

$$= PE(q, 2i)*PE(\Delta, 2i+1) + PE(q, 2i+1)*PE(\Delta, 2i)$$

$$
PE(p, 2i+1) = cos(p/M) = cos(\frac{q + \Delta}{M}) = cos(q/M) * cos(\Delta/M) - sin(q/M) * sin(\Delta/M)
$$

$$
= PE(q, 2i+1)*PE(\Delta, 2i+1) - PE(q, 2i)*PE(\Delta, 2i)
$$

### Podpunkt c)


\begin{bmatrix}
PE(p, 2i) \\
PE(p, 2i+1)
\end{bmatrix}

$$=$$


\begin{bmatrix}
cos\left(\frac{\Delta}{M}\right) & sin\left(\frac{\Delta}{M}\right) \\
-sin\left(\frac{\Delta}{M}\right) & cos\left(\frac{\Delta}{M}\right)
\end{bmatrix}

$$*$$

\begin{bmatrix}
PE(q, 2i) \\
PE(q, 2i+1)
\end{bmatrix}

Jest to przydatne poniewaz relacja miedzy tokenem p i q zawsze bedzie taka sama, niezaleznie od tego na jakiej pozycji jest.