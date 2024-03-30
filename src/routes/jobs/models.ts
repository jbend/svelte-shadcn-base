export type Job = {
  id: string;
  name: string;
  active: boolean;
  favorite: boolean;
};

export type CreateJob = {
  name: string;
};

export type UpdateJobName = {
  id: string;
  name: string;
};

export type UpdateJobActive = {
  id: string;
  active: boolean;
};

export type UpdateJobFavorite = {
  id: string;
  favorite: boolean;
};



