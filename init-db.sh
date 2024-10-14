#!/bin/bash

echo "Aguardando o SQL Server iniciar..."
sleep 20
until /opt/mssql-tools/bin/sqlcmd -S sqlserver,1433 -U SA -P "SqlServer2019!" -Q "SELECT 1" &> /dev/null; do
    echo "SQL Server não está pronto. Aguardando..."
    sleep 5
done

echo "SQL Server está pronto. Executando o script SQL..."

/opt/mssql-tools/bin/sqlcmd -S sqlserver,1433 -U SA -P "SqlServer2019!" -d master -i /usr/src/app/CreateAndInitDatabase.sql

echo "Script SQL executado com sucesso."
