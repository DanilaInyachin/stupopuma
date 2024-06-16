import React, { FC, useContext, useEffect, useState } from 'react';
import { Typography, Box, Grid, Button } from '@mui/material';
import { useNavigate } from 'react-router-dom';
import CurrentUserContext from '../../contex';
import axios from 'axios';
import { FormProvider, SubmitHandler, useForm } from 'react-hook-form';
import FormInput from '../../components/FormInput';

interface IUserInfoResponse {
  data: IUserInfo;
  status: number;
}

interface IUserInfo {
  lastname: string;
  firstname: string;
  surname: string;
  role: string;
}

const initialState: IUserInfo = {
  lastname: '',
  firstname: '',
  surname: '',
  role: '',
} 

const GeneralInfo: FC = () => {
  const [isEditable, setIsEditable] = useState(false);
  const [data, setData] = useState<IUserInfo>(initialState);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);

  useEffect(() => {
    if (!context || !context.isAuthAndToken) {
      navigate('/signin');
    } else {
      axios
        .post('//localhost:8080/view_user', {
          token: context.isAuthAndToken,
        })
        .then((response) => {
          console.log(response);
          setData({
            lastname: response.data.lastname,
            firstname: response.data.firstname,
            surname: response.data.surname,
            role: response.data.role,
          })
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  });

  const methods = useForm<IUserInfo>({
    defaultValues: {
      ...data
    },
  });

  const handleEditClick = () => {
    setIsEditable(true);
  };

  const handleCancelClick = () => {
    setIsEditable(false);
    methods.reset(); // Reset form values to default
  };

  const handleSaveClick: SubmitHandler<IUserInfo> = (data) => {
    console.log('Saved data:', data);
    setIsEditable(false);
  };

  return (
    <Box>
      <Typography variant="h6" component="h2" sx={{ mb: 2 }}>
        General Information
      </Typography>
      <FormProvider {...methods}>
        <Box
          component="form"
          onSubmit={methods.handleSubmit(handleSaveClick)}
          noValidate
          autoComplete="off"
        >
          <FormInput
            name="lastname"
            label="Lastname"
            disabled={!isEditable}
          />
          <FormInput
            name="firstname"
            label="Firstname"
            disabled={!isEditable}
          />
          <FormInput
            name="surname"
            label="Surname"
            disabled={!isEditable}
          />
          <FormInput
            name="role"
            label="Role"
            disabled
          />
          <Grid container spacing={2} sx={{ mt: 2 }}>
            {isEditable ? (
              <>
                <Grid item>
                  <Button variant="contained" color="primary" type="submit">
                    Save
                  </Button>
                </Grid>
                <Grid item>
                  <Button variant="outlined" color="secondary" onClick={handleCancelClick}>
                    Cancel
                  </Button>
                </Grid>
              </>
            ) : (
              <Grid item>
                <Button variant="contained" color="primary" onClick={handleEditClick}>
                  Edit
                </Button>
              </Grid>
            )}
          </Grid>
        </Box>
      </FormProvider>
    </Box>
  );
};

export default GeneralInfo;
