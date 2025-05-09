// pages/api/gpus.ts
import { PrismaClient } from '@prisma/client';
import type { NextApiRequest, NextApiResponse } from 'next';

const prisma = new PrismaClient();

interface GPU {
  id: string;      // If you use `id` as string (e.g., cuid), leave it as is.
  name: string;
  status: string;
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<{ gpus: GPU[] } | { error: string }>
) {
  if (req.method === 'GET') {
    try {
      // Get all GPUs from the database
      const gpus = await prisma.gPU.findMany();  // Ensure it's 'GPU' as per your model name
      res.status(200).json({ gpus });
    } catch (error) {
      // If error occurs, send 500 response with the error message
      res.status(500).json({ error: 'Failed to fetch GPUs' });
    }
  } else {
    // For unsupported methods like POST, PUT, DELETE
    res.status(405).json({ error: 'Method Not Allowed' });
  }
}
