/*
Navicat MySQL Data Transfer

Source Server         : 192.168.0.126-mysql
Source Server Version : 50726
Source Host           : 192.168.0.126:29010
Source Database       : invengodbv4

Target Server Type    : MYSQL
Target Server Version : 50726
File Encoding         : 65001

Date: 2024-12-06 12:18:25
*/

SET FOREIGN_KEY_CHECKS=0;

-- ----------------------------
-- Table structure for libitem
-- ----------------------------
DROP TABLE IF EXISTS `libitem`;
CREATE TABLE `libitem` (
  `Id` varchar(32) NOT NULL,
  `CreationTime` datetime(6) NOT NULL,
  `CreatorUserId` bigint(20) DEFAULT NULL,
  `LastModificationTime` datetime(6) DEFAULT NULL,
  `LastModifierUserId` bigint(20) DEFAULT NULL,
  `IsDeleted` bit(1) NOT NULL,
  `DeleterUserId` bigint(20) DEFAULT NULL,
  `DeletionTime` datetime(6) DEFAULT NULL,
  `InfoId` varchar(32) DEFAULT NULL,
  `Title` varchar(512) NOT NULL,
  `Author` varchar(512) DEFAULT NULL,
  `Barcode` varchar(32) NOT NULL,
  `IsEnable` bit(1) NOT NULL,
  `CallNo` varchar(64) DEFAULT NULL,
  `PreCallNo` varchar(64) DEFAULT NULL,
  `CatalogCode` varchar(32) DEFAULT NULL,
  `ItemState` tinyint(3) unsigned NOT NULL,
  `PressmarkId` varchar(32) DEFAULT NULL,
  `PressmarkName` varchar(64) DEFAULT NULL,
  `LocationId` varchar(32) DEFAULT NULL,
  `LocationName` varchar(128) DEFAULT NULL,
  `BookBarcode` varchar(32) DEFAULT NULL,
  `ISBN` varchar(32) DEFAULT NULL,
  `PubNo` smallint(6) DEFAULT NULL,
  `Publisher` varchar(512) DEFAULT NULL,
  `PubDate` varchar(512) DEFAULT NULL,
  `Price` varchar(32) DEFAULT NULL,
  `Pages` varchar(32) DEFAULT NULL,
  `Summary` longtext,
  `ItemType` tinyint(3) unsigned NOT NULL,
  `Remark` varchar(256) DEFAULT NULL,
  `OriginType` tinyint(3) unsigned NOT NULL,
  `CreateType` tinyint(3) unsigned NOT NULL,
  `TenantId` int(11) NOT NULL,
  PRIMARY KEY (`Id`),
  KEY `IX_LibItem_TenantId_IsDeleted_Barcode` (`TenantId`,`IsDeleted`,`Barcode`),
  KEY `IX_LibItem_TenantId_IsDeleted_CreationTime` (`TenantId`,`IsDeleted`,`CreationTime`),
  FULLTEXT KEY `IX_LibItem_Book_Search` (`Title`,`Author`) /*!50100 WITH PARSER `ngram` */ 
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
