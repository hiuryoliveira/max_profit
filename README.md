# LUCRO MÁXIMO DA AÇÃO

🎯 OBJETIVO
O objetivo do challenge é encontrar o maior lucro possível que pode ser obtido comprando e vendendo uma ação ao longo de vários dias, seguindo as regras abaixo.

👨‍⚖️ REGRAS
Você só pode comprar e/ou vender a ação uma vez por dia.
Você só pode ter uma ação em mãos por vez.
Você pode comprar e vender a ação no mesmo dia.

💡 EXEMPLOS
Exemplo 1:

```
const prices = [7, 1, 5, 3, 6, 4] 
Output: 7
/*
Explicação:
Compre a ação no dia 2 (preço = 1) e venda no dia 3 (preço = 5), lucro = 5-1 = 4.
Compre a ação no dia 4 (preço = 3) e venda no dia 5 (preço = 6), lucro = 6-3 = 3.
Lucro total = 4 + 3 = 7.
*/
```


Exemplo 2:

```
const prices = [1, 2, 3, 4, 5] 
Output: 4
/*
Explicação:
Compre a ação no dia 1 (preço = 1) e venda no dia 5 (preço = 5), lucro = 5-1 = 4.
Lucro total = 4.
*/

```


Exemplo 3:
```
const prices = [7, 6, 4, 3, 1] 
Output: 0
/*
Explicação:
Não há como obter um lucro positivo, portanto, nunca compramos a ação para obter o lucro máximo de 0.
*/
```

