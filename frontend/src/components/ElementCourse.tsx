import React, { FC, useContext, useEffect, useState } from 'react';
import {
  Box,
  List,
  ListItem,
  ListItemText,
  Collapse,
  Button,
  Typography,
  TextField,
} from '@mui/material';
import { ExpandLess, ExpandMore } from '@mui/icons-material';
import CurrentUserContext from '../context';
import axios from 'axios';
import { useTranslation } from 'react-i18next';

interface ICourse {
  namecourses: string;
  id: number;
  nametheme: string;
}

interface ElementCourseProps {
  namecourse: string;
  needButton?: boolean;
  isTeacher?: boolean;
}

const ElementCourse: FC<ElementCourseProps> = ({
  namecourse,
  needButton = true,
  isTeacher = false,
}) => {
  const [open, setOpen] = useState(false);
  const [courses, setCourses] = useState<ICourse[]>([]);
  const context = useContext(CurrentUserContext);
  const [editCourseName, setEditCourseName] = useState(namecourse);
  const [isEditingCourse, setIsEditingCourse] = useState(false);
  const [isAddingTopic, setIsAddingTopic] = useState(false);
  const [newTopicName, setNewTopicName] = useState('');
  const { t } = useTranslation();

  useEffect(() => {
    if (context) {
      axios
        .post('//localhost:8080/get_topics_by_course', {
          namecourses: namecourse,
        })
        .then((response) => {
          console.log(response);
          setCourses(response.data);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  }, [context, namecourse, setCourses]);

  const handleToggle = () => {
    setOpen((prevOpen) => !prevOpen);
  };

  const handleEditCourse = () => {
    if (context && context.isAuthAndToken) {
      axios
        .post('//localhost:8080/edit_course', {
          token: context.isAuthAndToken,
          new_name_courses: editCourseName,
          name_courses: namecourse,
        })
        .then((response) => {
          console.log(response);
          setIsEditingCourse(false);
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  };

  const handleAddTopic = () => {
    if (context && context.isAuthAndToken) {
      axios
        .put('//localhost:8080/add_prepod_courses', {
          token: context.isAuthAndToken,
          nameCourses: namecourse,
          nametheme: newTopicName,
        })
        .then((response) => {
          console.log(response);
          setIsAddingTopic(false);
          setNewTopicName('');
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  };

  const handleEnroll = () => {
    if (context && context.isAuthAndToken) {
      axios
        .post('//localhost:8080/register_user_courses', {
          nameCourses: namecourse,
          token: context.isAuthAndToken,
        })
        .catch((error) => {
          console.error('Error: ', error);
        });
    }
  };

  const handleEditTopic = () => {
    console.log('Edit topic');
  };

  return (
    <Box>
      <ListItem onClick={handleToggle}>
        <ListItemText primary={namecourse} />
        {open ? <ExpandLess /> : <ExpandMore />}
      </ListItem>
      <Collapse in={open} timeout="auto" unmountOnExit>
        <List component="div" disablePadding>
          {courses.map((course) => (
            <ListItem key={course.id} sx={{ pl: 4 }}>
              <ListItemText primary={course.nametheme} />
            </ListItem>
          ))}
        </List>
        {needButton ? (
          <Box sx={{ display: 'flex', justifyContent: 'flex-end', p: 2 }}>
            <Button variant="contained" color="primary" onClick={handleEnroll}>
              Enroll
            </Button>
          </Box>
        ) : null}
        {isTeacher && (
          <Box sx={{ p: 2 }}>
            <Button
              variant="outlined"
              color="primary"
              onClick={() => setIsEditingCourse(true)}
            >
              {t('Change course')}
            </Button>
            <Button
              variant="outlined"
              color="primary"
              onClick={() => setIsAddingTopic(true)}
            >
              {t('Add theme')}
            </Button>
            <Button variant="outlined" color="primary">
              {t('Change theme')}
            </Button>
            {isEditingCourse && (
              <Box sx={{ display: 'flex', flexDirection: 'column', mt: 2 }}>
                <TextField
                  label={t('Name of course')}
                  variant="outlined"
                  value={editCourseName}
                  onChange={(e) => setEditCourseName(e.target.value)}
                  // onKeyPress={(e) => {
                  //   if (e.key === 'Enter') handleEditCourse();
                  // }}
                />
                <Button
                  variant="contained"
                  color="primary"
                  onClick={handleEditCourse}
                  sx={{ mt: 2 }}
                >
                  {t('Enter')}
                </Button>
              </Box>
            )}
            {isAddingTopic && (
              <Box sx={{ display: 'flex', flexDirection: 'column', mt: 2 }}>
                <TextField
                  label={t('Name of theme')}
                  variant="outlined"
                  value={newTopicName}
                  onChange={(e) => setNewTopicName(e.target.value)}
                  // onKeyPress={(e) => {
                  //   if (e.key === 'Enter') handleAddTopic();
                  // }}
                />
                <Button
                  variant="contained"
                  color="primary"
                  onClick={handleAddTopic}
                  sx={{ mt: 2 }}
                >
                  {t('Enter')}
                </Button>
              </Box>
            )}
          </Box>
        )}
      </Collapse>
    </Box>
  );
};

export default ElementCourse;
