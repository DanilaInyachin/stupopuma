import React, { FC, useEffect, useState } from 'react';
import { Typography, Box } from '@mui/material';

interface IUserInfoResponse {
  data: IUserInfo,
  status: number,
}

interface IUserInfo {
  lastname: string;
  firstname: string;
  surname: string;
  role: string;
}

const GeneralInfo: FC = () => {
  const [isEditable, setIsEditable] = useState(false);
  const [data, setData] = useState<IUserInfo>();
  useEffect(() => {
    try {
      const response = await axios.get('//localhost:8080/view_user', {
        mail: values.email,
        password: values.password,
      });
      console.log(response.data);
      navigate('/profile');
    } catch (error) {
      console.error('Error: ', error);
    }
  };
  })

  return (
    <Box>
      <Typography variant="h6" component="h2">
        General Information
      </Typography>
      <Typography variant="body1">
        Here is some general information about the user.
      </Typography>
    </Box>
  );
};

export default GeneralInfo;
