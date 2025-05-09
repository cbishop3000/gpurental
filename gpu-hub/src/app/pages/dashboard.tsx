import { useQuery } from 'react-query';
import axios from 'axios';

// Define the types for GPU and the API response
interface GPU {
  id: number;
  name: string;
  status: string;
}

interface GpuData {
  gpus: GPU[];
}

const fetchGpuData = async (): Promise<GpuData> => {
  const response = await axios.get('http://localhost:5000/api/gpus');
  return response.data;  // We assume the response contains an array of GPUs in 'gpus'
};

const Dashboard = () => {
  const { data, isLoading, error } = useQuery<GpuData, Error>('gpuData', fetchGpuData);

  if (isLoading) return <div>Loading...</div>;
  if (error instanceof Error) return <div>Error: {error.message}</div>;

  return (
    <div>
      <h1>Dashboard</h1>
      <h2>Available GPUs:</h2>
      <ul>
        {data?.gpus.map((gpu) => (
          <li key={gpu.id}>
            {gpu.name} - {gpu.status}
          </li>
        ))}
      </ul>
    </div>
  );
};

export default Dashboard;
