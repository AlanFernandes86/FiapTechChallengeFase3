USE [master]
GO
IF EXISTS (SELECT name FROM sys.databases WHERE name = 'TechChallenge')
BEGIN
    ALTER DATABASE TechChallenge SET SINGLE_USER WITH ROLLBACK IMMEDIATE;
    DROP DATABASE TechChallenge;
END


USE [master]
GO
/****** Object:  Database [TechChallenge]    Script Date: 1/28/2024 20:15:30 ******/
CREATE DATABASE [TechChallenge]
 CONTAINMENT = NONE
 ON  PRIMARY 
( NAME = N'TechChallenge', FILENAME = N'/var/opt/mssql/data/TechChallenge.mdf' , SIZE = 8192KB , MAXSIZE = UNLIMITED, FILEGROWTH = 65536KB )
 LOG ON 
( NAME = N'TechChallenge_log', FILENAME = N'/var/opt/mssql/data/TechChallenge_log.ldf' , SIZE = 8192KB , MAXSIZE = 2048GB , FILEGROWTH = 65536KB )
 WITH CATALOG_COLLATION = DATABASE_DEFAULT
GO
ALTER DATABASE [TechChallenge] SET COMPATIBILITY_LEVEL = 150
GO
IF (1 = FULLTEXTSERVICEPROPERTY('IsFullTextInstalled'))
begin
EXEC [TechChallenge].[dbo].[sp_fulltext_database] @action = 'enable'
end
GO
ALTER DATABASE [TechChallenge] SET ANSI_NULL_DEFAULT OFF 
GO
ALTER DATABASE [TechChallenge] SET ANSI_NULLS OFF 
GO
ALTER DATABASE [TechChallenge] SET ANSI_PADDING OFF 
GO
ALTER DATABASE [TechChallenge] SET ANSI_WARNINGS OFF 
GO
ALTER DATABASE [TechChallenge] SET ARITHABORT OFF 
GO
ALTER DATABASE [TechChallenge] SET AUTO_CLOSE OFF 
GO
ALTER DATABASE [TechChallenge] SET AUTO_SHRINK OFF 
GO
ALTER DATABASE [TechChallenge] SET AUTO_UPDATE_STATISTICS ON 
GO
ALTER DATABASE [TechChallenge] SET CURSOR_CLOSE_ON_COMMIT OFF 
GO
ALTER DATABASE [TechChallenge] SET CURSOR_DEFAULT  GLOBAL 
GO
ALTER DATABASE [TechChallenge] SET CONCAT_NULL_YIELDS_NULL OFF 
GO
ALTER DATABASE [TechChallenge] SET NUMERIC_ROUNDABORT OFF 
GO
ALTER DATABASE [TechChallenge] SET QUOTED_IDENTIFIER OFF 
GO
ALTER DATABASE [TechChallenge] SET RECURSIVE_TRIGGERS OFF 
GO
ALTER DATABASE [TechChallenge] SET  DISABLE_BROKER 
GO
ALTER DATABASE [TechChallenge] SET AUTO_UPDATE_STATISTICS_ASYNC OFF 
GO
ALTER DATABASE [TechChallenge] SET DATE_CORRELATION_OPTIMIZATION OFF 
GO
ALTER DATABASE [TechChallenge] SET TRUSTWORTHY OFF 
GO
ALTER DATABASE [TechChallenge] SET ALLOW_SNAPSHOT_ISOLATION OFF 
GO
ALTER DATABASE [TechChallenge] SET PARAMETERIZATION SIMPLE 
GO
ALTER DATABASE [TechChallenge] SET READ_COMMITTED_SNAPSHOT OFF 
GO
ALTER DATABASE [TechChallenge] SET HONOR_BROKER_PRIORITY OFF 
GO
ALTER DATABASE [TechChallenge] SET RECOVERY FULL 
GO
ALTER DATABASE [TechChallenge] SET  MULTI_USER 
GO
ALTER DATABASE [TechChallenge] SET PAGE_VERIFY CHECKSUM  
GO
ALTER DATABASE [TechChallenge] SET DB_CHAINING OFF 
GO
ALTER DATABASE [TechChallenge] SET FILESTREAM( NON_TRANSACTED_ACCESS = OFF ) 
GO
ALTER DATABASE [TechChallenge] SET TARGET_RECOVERY_TIME = 60 SECONDS 
GO
ALTER DATABASE [TechChallenge] SET DELAYED_DURABILITY = DISABLED 
GO
ALTER DATABASE [TechChallenge] SET ACCELERATED_DATABASE_RECOVERY = OFF  
GO
EXEC sys.sp_db_vardecimal_storage_format N'TechChallenge', N'ON'
GO
ALTER DATABASE [TechChallenge] SET QUERY_STORE = OFF
GO

USE [TechChallenge]
GO
/****** Object:  Table [dbo].[client]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[client](
	[cpf] [varchar](11) NOT NULL,
	[name] [varchar](100) NULL,
	[email] [varchar](100) NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_client] PRIMARY KEY CLUSTERED 
(
	[cpf] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[order]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[order](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[order_status_id] [int] NOT NULL,
	[client_cpf] [varchar](11) NULL,
	[client_name] [varchar](50) NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_order] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[order_product]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[order_product](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[product_id] [int] NOT NULL,
	[order_id] [int] NOT NULL,
	[quantity] [int] NOT NULL,
	[price] [float] NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_order_product] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[order_status]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[order_status](
	[id] [int] NOT NULL,
	[name] [varchar](50) NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_order_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[payment]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[payment](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[order_id] [int] NOT NULL,
	[payment_status_id] [int] NOT NULL,
	[payment_method_id] [int] NULL,
	[payment_method_order_id] [varchar](100) NOT NULL,
	[value] [float] NOT NULL,
	[message] [varchar](100) NOT NULL,
	[origin] [varchar](100) NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_payment] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
/****** Object:  Table [dbo].[product]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[product](
	[id] [int] IDENTITY(1,1) NOT NULL,
	[product_category_id] [int] NOT NULL,
	[name] [varchar](100) NOT NULL,
	[description] [varchar](500) NOT NULL,
	[price] [float] NOT NULL,
	[image_url] [varchar](1000) NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_product_1] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO

/****** Object:  Table [dbo].[payment_status]    Script Date: 05/10/2024 15:52:47 ******/
SET ANSI_NULLS ON
GO

SET QUOTED_IDENTIFIER ON
GO

CREATE TABLE [dbo].[payment_status](
	[id] [int] NOT NULL,
	[name] [varchar](50) NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_payment_status] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO

/****** Object:  Table [dbo].[product_category]    Script Date: 1/28/2024 20:15:30 ******/
SET ANSI_NULLS ON
GO
SET QUOTED_IDENTIFIER ON
GO
CREATE TABLE [dbo].[product_category](
	[id] [int] NOT NULL,
	[name] [varchar](50) NOT NULL,
	[description] [varchar](500) NOT NULL,
	[updated_at] [datetime] NOT NULL,
	[created_at] [datetime] NOT NULL,
 CONSTRAINT [PK_product_category_1] PRIMARY KEY CLUSTERED 
(
	[id] ASC
)WITH (PAD_INDEX = OFF, STATISTICS_NORECOMPUTE = OFF, IGNORE_DUP_KEY = OFF, ALLOW_ROW_LOCKS = ON, ALLOW_PAGE_LOCKS = ON, OPTIMIZE_FOR_SEQUENTIAL_KEY = OFF) ON [PRIMARY]
) ON [PRIMARY]
GO
ALTER TABLE [dbo].[order]  WITH CHECK ADD  CONSTRAINT [FK_order_client] FOREIGN KEY([client_cpf])
REFERENCES [dbo].[client] ([cpf])
GO
ALTER TABLE [dbo].[order] CHECK CONSTRAINT [FK_order_client]
GO
ALTER TABLE [dbo].[order]  WITH CHECK ADD  CONSTRAINT [FK_order_order_status] FOREIGN KEY([order_status_id])
REFERENCES [dbo].[order_status] ([id])
GO
ALTER TABLE [dbo].[order] CHECK CONSTRAINT [FK_order_order_status]
GO
ALTER TABLE [dbo].[order_product]  WITH CHECK ADD  CONSTRAINT [FK_order_product_order] FOREIGN KEY([order_id])
REFERENCES [dbo].[order] ([id])
GO
ALTER TABLE [dbo].[order_product] CHECK CONSTRAINT [FK_order_product_order]
GO
ALTER TABLE [dbo].[order_product]  WITH CHECK ADD  CONSTRAINT [FK_order_product_product] FOREIGN KEY([product_id])
REFERENCES [dbo].[product] ([id])
GO
ALTER TABLE [dbo].[order_product] CHECK CONSTRAINT [FK_order_product_product]
GO
ALTER TABLE [dbo].[payment]  WITH CHECK ADD  CONSTRAINT [FK_payment_order] FOREIGN KEY([order_id])
REFERENCES [dbo].[order] ([id])
GO
ALTER TABLE [dbo].[payment] CHECK CONSTRAINT [FK_payment_order]
GO
ALTER TABLE [dbo].[product]  WITH CHECK ADD  CONSTRAINT [FK_product_product_category] FOREIGN KEY([product_category_id])
REFERENCES [dbo].[product_category] ([id])
GO
ALTER TABLE [dbo].[product] CHECK CONSTRAINT [FK_product_product_category]
GO
USE [master]
GO
ALTER DATABASE [TechChallenge] SET  READ_WRITE 
GO

USE [TechChallenge]
GO

INSERT INTO [dbo].[order_status]
           ([id]
           ,[name]
           ,[updated_at]
           ,[created_at])
     VALUES
           (1, 'CREATED', GETDATE(), GETDATE()),
		   (2, 'RECEIVED', GETDATE(), GETDATE()),
		   (4, 'IN_PREPARATION', GETDATE(), GETDATE()),
		   (8, 'READY', GETDATE(), GETDATE()),
		   (16, 'COMPLETED', GETDATE(), GETDATE()),
		   (32, 'CANCELLED', GETDATE(), GETDATE()),
		   (14, 'ACTIVE', GETDATE(), GETDATE())
GO

USE [TechChallenge]
GO

INSERT INTO [dbo].[product_category]
           ([id]
           ,[name]
		   ,[description]
           ,[updated_at]
           ,[created_at])
     VALUES
           (0, 'LANCHE', '', GETDATE(), GETDATE()),
		   (1, 'ACOMPANHAMENTO', '', GETDATE(), GETDATE()),
		   (2, 'BEBIDA', '', GETDATE(), GETDATE()),
		   (3, 'SOBREMESA', '', GETDATE(), GETDATE())
GO

USE [TechChallenge]
GO

INSERT INTO [dbo].[product]
           ([product_category_id]
           ,[name]
           ,[description]
		   ,[image_url]
           ,[price]
           ,[updated_at]
           ,[created_at])
     VALUES
           (0,'X-Tudo','Lanche completo!','',15.89,GETDATE(),GETDATE()),
		   (0,'Pizza Brotinho','Diversos sabores!','',13.29,GETDATE(),GETDATE()),
		   (1,'Batata Frita P','Generosa pequena porção!','',10.50,GETDATE(),GETDATE()),
		   (2,'Refrigerante Lata','Diversos sabores!','',7.0,GETDATE(),GETDATE()),
		   (3,'Petit Gateau','Bolo de chocolate com sorvete!','',19.0,GETDATE(),GETDATE())
GO

USE [TechChallenge]
GO

INSERT INTO [dbo].[client]
           ([cpf]
           ,[name]
           ,[email]
           ,[updated_at]
           ,[created_at])
     VALUES
           (29642467003,'João Apolinário','apo.joao@email.com',GETDATE(),GETDATE()),
		   (67723050003,'Peter Parker','email.secreto@aranha.com',GETDATE(),GETDATE()),
		   (0,'Anonimo','email@anonimo.com',GETDATE(),GETDATE())
GO

INSERT INTO [dbo].[payment_status]
           ([id]
           ,[name]
           ,[updated_at]
           ,[created_at])
     VALUES
		   (1, 'PENDING', GETDATE(), GETDATE()),
		   (2, 'PAID', GETDATE(), GETDATE()),
		   (4, 'CANCELLED', GETDATE(), GETDATE())
GO

