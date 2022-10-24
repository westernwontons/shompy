/*
  Warnings:

  - Made the column `amount` on table `Food` required. This step will fail if there are existing NULL values in that column.
  - Made the column `total` on table `Food` required. This step will fail if there are existing NULL values in that column.

*/
-- AlterTable
ALTER TABLE "Food" ALTER COLUMN "amount" SET NOT NULL,
ALTER COLUMN "total" SET NOT NULL;
