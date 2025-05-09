"use client"
import { useEffect, useState } from "react";

// Define the GPU type
interface GPU {
  id: string;
  name: string;
  status: string;
}

export default function Home() {
  const [gpus, setGpus] = useState<GPU[]>([]); // State to store GPUs
  const [loading, setLoading] = useState<boolean>(true); // Loading state
  const [error, setError] = useState<string | null>(null); // Error state

  useEffect(() => {
    // Fetch GPU data from the API
    const fetchGpus = async () => {
      try {
        const response = await fetch("/api/gpus");
        const data = await response.json();
        setGpus(data.gpus); // Set the GPU data
        setLoading(false); // Stop loading once data is fetched
      } catch (error) {
        setError("Failed to load GPU data"); // Handle error
        setLoading(false); // Stop loading if an error occurs
      }
    };

    fetchGpus(); // Call the function to fetch data
  }, []);

  return (
    <div className="min-h-screen p-8 flex flex-col justify-center items-center gap-8">
      {/* GPU Data Section */}
      {loading ? (
        <div>Loading GPUs...</div>
      ) : error ? (
        <div>{error}</div>
      ) : (
        <div className="flex flex-col gap-4">
          <h2 className="text-xl font-semibold">Available GPUs:</h2>
          <ul className="list-none">
            {gpus.map((gpu) => (
              <li key={gpu.id} className="border p-4 rounded-lg">
                <h3 className="text-lg">{gpu.name}</h3>
                <p>Status: {gpu.status}</p>
              </li>
            ))}
          </ul>
        </div>
      )}
    </div>
  );
}
