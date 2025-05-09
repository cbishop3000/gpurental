/*
  Warnings:

  - The primary key for the `GPU` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `createdAt` on the `GPU` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE "GPU" DROP CONSTRAINT "GPU_pkey",
DROP COLUMN "createdAt",
ALTER COLUMN "id" DROP DEFAULT,
ALTER COLUMN "id" SET DATA TYPE TEXT,
ADD CONSTRAINT "GPU_pkey" PRIMARY KEY ("id");
DROP SEQUENCE "GPU_id_seq";
