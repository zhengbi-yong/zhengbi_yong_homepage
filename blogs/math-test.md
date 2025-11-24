---
title: "数学公式测试页面"
date: "2025-11-24"
tags: ["Math", "Test", "LaTeX"]
categories: ["Test"]
---

# Math Support Test Suite

## 1. Fundamentals
Inline math: $a^2 + b^2 = c^2$ and $E = mc^2$.
Subscripts and Superscripts: $x_i$, $x^2$, $e^{i\pi}$, $A_{i,j}$.

## 2. Greek Letters
Lowercase: 
$\alpha \beta \gamma \delta \epsilon \varepsilon \zeta \eta \theta \vartheta \iota \kappa \lambda \mu \nu \xi o \pi \varpi \rho \varrho \sigma \varsigma \tau \upsilon \phi \varphi \chi \psi \omega$

Uppercase:
$\Gamma \Delta \Theta \Lambda \Xi \Pi \Sigma \Upsilon \Phi \Psi \Omega$

## 3. Operators & Relations
Binary: $+, -, \times, \cdot, \pm, \mp, \div, \ast, \star, \circ, \bullet$
Relations: $=, \neq, <, >, \leq, \geq, \equiv, \sim, \approx, \cong, \propto$
Logic: $\forall, \exists, \nexists, \neg, \land, \lor, \implies, \iff$
Sets: $\in, \notin, \subset, \subseteq, \cup, \cap, \setminus, \emptyset$

## 4. Big Operators
Summation: $\sum_{i=1}^n i^2 = \frac{n(n+1)(2n+1)}{6}$
Product: $\prod_{i=1}^\infty \left(1 - \frac{1}{p_i^2}\right) = \frac{6}{\pi^2}$
Integral: $\int_{-\infty}^\infty e^{-x^2} dx = \sqrt{\pi}$
Contour Integral: $\oint_C \vec{F} \cdot d\vec{r}$

## 5. Fractions, Roots, Calculus
Fractions: $\frac{a}{b}, \frac{\partial y}{\partial x}, \frac{d^2y}{dx^2}$
Roots: $\sqrt{x}, \sqrt[3]{x}, \sqrt{1+\sqrt{x}}$
Limits: $\lim_{x \to 0} \frac{\sin x}{x} = 1$
Derivatives: $f'(x), f''(x), \dot{x}, \ddot{x}, \nabla f$

## 6. Matrices & Brackets
Matrix (pmatrix):
$$
\begin{pmatrix}
1 & 2 & 3 \\\\
4 & 5 & 6 \\\\
7 & 8 & 9
\end{pmatrix}
$$
Matrix (bmatrix):
$$
\begin{bmatrix}
a & b \\\\
c & d
\end{bmatrix}
$$
Cases:
$$
f(x) = \begin{cases}
x^2 & \text{if } x > 0 \\\\
-x & \text{if } x \le 0
\end{cases}
$$
Delimiters: $\left( \frac{1}{2} \right), \left[ \frac{a}{b} \right], \left\\{ \sum_{i=1}^n a_i \right\\}, \langle \psi | \phi \rangle$

## 7. Fonts & Accents
Blackboard: $\mathbb{R}, \mathbb{C}, \mathbb{Z}, \mathbb{N}, \mathbb{Q}$
Calligraphic: $\mathcal{A}, \mathcal{B}, \mathcal{L}, \mathcal{H}$
Fraktur: $\mathfrak{g}, \mathfrak{so}(3)$
Accents: $\hat{a}, \bar{a}, \tilde{a}, \vec{a}, \dot{a}, \ddot{a}$
Space: $a\,b\:c\;d\quad e\qquad f\!g$

## 8. Chemistry (via copy-tex extension if enabled, else just text)
$\text{H}_2\text{O}, \text{SO}_4^{2-}$

## 9. Complex Equation
Maxwell's Equations (Differential form):
$$
\begin{aligned}
\nabla \cdot \mathbf{E} &= \frac{\rho}{\varepsilon_0} \\\\
\nabla \cdot \mathbf{B} &= 0 \\\\
\nabla \times \mathbf{E} &= -\frac{\partial \mathbf{B}}{\partial t} \\\\
\nabla \times \mathbf{B} &= \mu_0\mathbf{J} + \mu_0\varepsilon_0\frac{\partial \mathbf{E}}{\partial t}
\end{aligned}
$$

