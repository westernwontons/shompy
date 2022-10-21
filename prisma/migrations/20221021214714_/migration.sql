/*
  Warnings:

  - You are about to drop the `User` table. If the table is not empty, all the data it contains will be lost.

*/
-- DropTable
DROP TABLE "User";

-- CreateTable
CREATE TABLE "Food" (
    "id" SERIAL NOT NULL,
    "food_type" VARCHAR(20) NOT NULL,
    "name" VARCHAR(200) NOT NULL,
    "price" REAL NOT NULL,
    "date" DATE NOT NULL,

    CONSTRAINT "Food_pkey" PRIMARY KEY ("id")
);
