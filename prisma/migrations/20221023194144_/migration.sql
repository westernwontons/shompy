/*
  Warnings:

  - You are about to alter the column `name` on the `Food` table. The data in that column could be lost. The data in that column will be cast from `VarChar(200)` to `VarChar(100)`.

*/
-- AlterTable
ALTER TABLE "Food" ALTER COLUMN "food_type" SET DATA TYPE VARCHAR(100),
ALTER COLUMN "name" SET DATA TYPE VARCHAR(100);
