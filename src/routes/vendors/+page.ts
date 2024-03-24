import type { PageLoad } from './$types';

export type Vendor = {
  id: number;
  name: string;
  create_at: string;
};

export type Payment = {
  id: string;
  amount: number;
  status: "pending" | "processing" | "success" | "failed";
  email: string;
};

export const load: PageLoad = () => {

  const payments: Payment[] = [
   {
      id: "m5gr84i9",
      amount: 16.8,
      status: "success",
      email: "ken99@yahoo.com",
    },
    {
      id: "m5gr54i9",
      amount: 416,
      status: "success",
      email: "bob99@yahoo.com",
    },
    {
      id: "m5gr54i9",
      amount: 416,
      status: "success",
      email: "bob99@yahoo.com",
    },
    {
      id: "m5gr84i9",
      amount: 16.8,
      status: "success",
      email: "ken99@yahoo.com",
    },
    {
      id: "m5gr54i9",
      amount: 416,
      status: "success",
      email: "bob99@yahoo.com",
    },
    {
      id: "m5gr54i9",
      amount: 416,
      status: "success",
      email: "bob99@yahoo.com",
    },
  ];
  
  const vendors: Vendor[] = [
    {
      id: 1,
      name: 'Vendor 1',
      create_at: '2021-01-21',
    },
    {
      id: 2,
      name: 'Vendor 2',
      create_at: '2021-01-01',
    },
  ];

	return {
    payments,
    vendors,
	};
};