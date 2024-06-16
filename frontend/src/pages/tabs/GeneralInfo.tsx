import React, { FC } from 'react';
import { Typography, Box } from '@mui/material';

const GeneralInfo: FC = () => {
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
