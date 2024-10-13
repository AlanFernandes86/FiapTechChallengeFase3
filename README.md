# Fiap Tech Challenge - Fase 2

Projeto referente a entrega da fase 2 do Tech Challenge do curso Software Architecture Pós Tech - turma 8SOAT.

## Requisitos do negócio

O back-end deve permitir:

- Receber novos pedidos
- Consultar pedido realizado
- Consultar lista de pedidos ativos
- Atualizar status do pedido
- Adicionar diversos produtos a um pedido
- Opção de pagamento integrada para MVP
- Publicar updates dos status de ordens
- Acompanhar o status de cada pedido
- Cadastrar produtos por categoria

## Desenho da Arquitetura

![image](https://github.com/user-attachments/assets/047b0e48-f5ad-4d03-b957-0ec36f959671)

## Como executar o projeto no kubernetes local

Obs* Projeto configurado para executar localmente, para esse exemplo foi utilizado o Kubernetes do Docker Desktop.

1. Primeiro instale o Docker Desktop caso não tenha e em seguida habilite o Kubernetes:
![image](https://github.com/user-attachments/assets/bfcee42a-480a-42a2-9ff8-caab3e997c5d)

2. Realize o clone do projeto na branch main e navegue até a pasta.
3. Dentro da pasta da aplicação abra o terminal e execute os comandos.

```docker-compose up -d```  

Disponibilizará o Sql Server e Broker Kafka que serão consumidos pela aplicação.  

![image](https://github.com/user-attachments/assets/81a260ad-5e2b-46e3-abe6-3696c14b2279)

```kubectl apply -f k8s/```  

Implantará a aplicação no k8s local  

![image](https://github.com/user-attachments/assets/ac97034f-a5a1-49c7-ba5c-79ad79ed787f)  

![image](https://github.com/user-attachments/assets/458f66b1-418d-49bd-8a41-584a5bdc695c)

## Link da collection do postman para teste local
https://github.com/AlanFernandes86/FiapTechChallenge/blob/feature/fase2/TechChallenge.postman_collection.json
