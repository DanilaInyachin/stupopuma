import React, { FC, useContext, useEffect, useState } from 'react';
import {
  Typography,
  Box,
  Grid,
  Button,
  Dialog,
  DialogContent,
  DialogContentText,
  DialogActions,
} from '@mui/material';
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
};

const GeneralInfo: FC = () => {
  const [isEditable, setIsEditable] = useState(false);
  const [open, setOpen] = useState(false);
  const [data, setData] = useState<IUserInfo>(initialState);
  const navigate = useNavigate();
  const context = useContext(CurrentUserContext);

  const methods = useForm<IUserInfo>({
    defaultValues: initialState,
  });

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
          });
          methods.reset({
            lastname: response.data.lastname,
            firstname: response.data.firstname,
            surname: response.data.surname,
            role: response.data.role,
          });
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, navigate, methods, setData]);

  const handleEditClick = () => {
    setIsEditable(true);
  };

  const handleCancelClick = () => {
    setIsEditable(false);
    methods.reset(data); // Reset form values to default
  };

  const handleSaveClick = methods.handleSubmit((formData: IUserInfo) => {
    setData(formData);
    axios
      .put('//localhost:8080/change_user_data', {
        surname: formData.surname,
        firstname: formData.firstname,
        lastname: formData.lastname,
        token: context?.isAuthAndToken,
      })
      .catch((error) => {
        console.error('Error: ', error);
      });
    setIsEditable(false);
  });

  const handleDeleteClick = () => {
    setOpen(true);
  };

  const handleClose = () => {
    setOpen(false);
  };

  const handleConfirmDelete = () => {
    setOpen(false);
    axios
      .post('//localhost:8080/delete_user', {
        token: context?.isAuthAndToken,
      })
      .then((response) => {
        console.log(response);
        context?.setIsAuthAndToken(undefined);
        navigate('/signin');
      });
  };

  return (
    <Box>
      <Typography variant="h6" component="h2" sx={{ mb: 2 }}>
        General Information
      </Typography>
      <FormProvider {...methods}>
        <Box component="form" noValidate autoComplete="off">
          <FormInput
            name="lastname"
            label="Lastname"
            variant="standard"
            disabled={!isEditable}
          />
          <FormInput
            name="firstname"
            label="Firstname"
            variant="standard"
            disabled={!isEditable}
          />
          <FormInput
            name="surname"
            label="Surname"
            variant="standard"
            disabled={!isEditable}
          />
          <FormInput name="role" label="Role" variant="standard" disabled />
          <Grid container spacing={2} sx={{ mt: 2 }}>
            {isEditable ? (
              <>
                <Grid item>
                  <Button
                    variant="contained"
                    color="primary"
                    onClick={handleSaveClick}
                  >
                    Save
                  </Button>
                </Grid>
                <Grid item>
                  <Button
                    variant="outlined"
                    color="secondary"
                    onClick={handleCancelClick}
                  >
                    Cancel
                  </Button>
                </Grid>
              </>
            ) : (
              <Grid item>
                <Button
                  variant="contained"
                  color="primary"
                  onClick={handleEditClick}
                >
                  Edit
                </Button>
              </Grid>
            )}
            <Grid item>
              <Button
                variant="contained"
                color="error"
                onClick={handleDeleteClick}
              >
                Delete Account
              </Button>
            </Grid>
          </Grid>
        </Box>
      </FormProvider>

      <Dialog open={open} onClose={handleClose}>
        <DialogContent>
          <DialogContentText>
            Are you sure you want to delete your account? This action cannot be
            undone.
          </DialogContentText>
        </DialogContent>
        <DialogActions>
          <Button onClick={handleClose} color="primary">
            Cancel
          </Button>
          <Button onClick={handleConfirmDelete} color="error" autoFocus>
            Confirm
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
};

export default GeneralInfo;
