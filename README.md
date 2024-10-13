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

2. Realize o clone do projeto na branch main e navegue até a pasta raiz.
3. Dentro da pasta da aplicação execute o docker compose.

```docker-compose up -d```  

Disponibilizará o Sql Server e Broker Kafka que serão consumidos pela aplicação.  
![image](https://github.com/user-attachments/assets/81a260ad-5e2b-46e3-abe6-3696c14b2279)  

4. Para receber as notificações do MercadoLivre localmente nesse exemplo utilizamos o proxy ngrok.

Siga as instruções no link para realizar a instalação e gerar um endpoint: 
https://dashboard.ngrok.com/get-started/setup/windows  

![image](https://github.com/user-attachments/assets/89968b05-8581-4aa6-ba5f-fd7d2bab7ed1)  
Copie o endpoint gerado:
![image](https://github.com/user-attachments/assets/000fdc77-a061-4261-a1ed-a7ae9c7c1c2d)
Altere a base url da variável MERCADO_PAGO_NOTIFICATION_URL no arquivo deployment.yml na pasta k8s.  
Altere somente o que está antes de .../api/v1...
![image](https://github.com/user-attachments/assets/56aa6d26-ab23-4db9-b129-1e0768ef6e98)

5. Execute o companto para implatar a aplicação no k8s
   
```kubectl apply -f k8s/```  

Implantará a aplicação no k8s local  

![image](https://github.com/user-attachments/assets/ac97034f-a5a1-49c7-ba5c-79ad79ed787f)  

![image](https://github.com/user-attachments/assets/458f66b1-418d-49bd-8a41-584a5bdc695c)

## Link da collection do postman para teste local
https://github.com/AlanFernandes86/FiapTechChallenge/blob/feature/fase2/TechChallenge.postman_collection.json
